use std::error::Error;

use reqwest::Client;
use serde_json::Value;

use crate::weather::{prelude::*, measurements::Coordinates, parsing::arguments::{Argument, Current, Hourly}};

static FORECAST_ENDPOINT: &'static str = "https://api.open-meteo.com/v1/forecast?";

/// Struct to parse weather data
/// Coordinates are necesarry, but other options are just an option
/// Parsing will make the following assumptions: 
///     The date format will be in ISO 8601
///     If timezone is None, the request will be set with the timezone as auto
///     If units is None, the standard SI [Celsius, KM/H, MM] will be used
///     If forecast_days is None, the open-meteo default of 7 days will be used
#[derive(Debug, Clone)]
pub struct OpenMeteo {
    coordinates: Coordinates,
    current: Vec<Current>,
    hourly: Vec<Hourly>,
    units: Option<Units>,
    forecast_days: Option<u8>,
    /// If timezone is set to None, the auto will be used, meaning the timezone will be in the coordinates' local 
    timezone: Option<String>
}

impl OpenMeteo {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { 
            coordinates,
            current: Vec::new(),
            forecast_days: None,
            hourly: Vec::new(),
            timezone: None,
            units: None
        }
    }

    pub fn current<I>(mut self, args: I) -> Self
    where 
        I: IntoIterator<Item = Current>,
    {
        self.current.extend(args);
        self
    }

    pub fn hourly<I>(mut self, args: I) -> Self
    where 
        I: IntoIterator<Item = Hourly>,
    {
        self.hourly.extend(args);
        self
    }

    pub fn units(mut self, units: Units) -> Self {
        self.units = Some(units);
        self
    }

    /// The max days open-meteo documentates is 16 days
    pub fn forecast_days(mut self, days: u8) -> Self {
        self.forecast_days = Some(days.min(16));
        self
    }

    pub fn timezone(mut self, timezone: &str) -> Self {
        self.timezone = Some(timezone.to_string());
        self
    }
}

impl OpenMeteo {
    fn build_url(&self) -> String {
        let mut url = String::new();

        // Coordinates
        url.push_str(format!("latitude={}&longitude={}", self.coordinates.lat, self.coordinates.lng).as_str());
        
        // TZ
        let tz = match &self.timezone {
            Some(tz) => tz.replace("/", "%2F"),
            None => "auto".to_string()
        };
        url.push_str(format!("&timezone={}", tz).as_str());

        // Forecast days
        if self.forecast_days.is_some() {
            url.push_str(format!("&forecast_days={}", self.forecast_days.unwrap()).as_str());
        }

        // Current
        if !self.current.is_empty() {
            url.push_str("&current=");
            self.current.iter().for_each(|arg| {
                url.push_str(format!(",{}", arg.to_string()).as_str());
            });
        }

        // Hourly
        if !self.hourly.is_empty() {
            if !self.hourly.is_empty() {
                url.push_str("&hourly=");
                self.hourly.iter().for_each(|arg| {
                    url.push_str(format!(",{}", arg.to_string()).as_str());
                });
            }
        }

        // Units
        if let Some(units) = &self.units {
            // Only adding the parameters if they're not the default ones already

            if units.speed != Speed::Kmh {
                url.push_str(format!("&wind_speed_unit={}", &units.speed.to_string()).as_str());
            }

            if units.temperature == Temperature::Fahrenheit {
                url.push_str(format!("&temperature_unit={}", &units.temperature.to_string()).as_str());
            }

            if units.length == Length::Inch {
                url.push_str(format!("&precipitation_unit={}", units.length.to_string()).as_str());
            }
        }

        url
    }

    /// Will either return an error, or the body of the response as a string
    pub async fn parse(&self) -> Result<Value, Box<dyn Error>> {
        let client = Client::new();
        let url = format!("{}{}", FORECAST_ENDPOINT, self.build_url());
        let response = client.get(url).send().await?.error_for_status()?;
        let body = response.text().await?;
        
        let deserialized: Value = serde_json::from_str(&body)?;
        Ok(deserialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn url_validity() {
        let coordinates = Coordinates::new(50.0, 20.0);
        let units = Units::new(Speed::Knots, Temperature::Fahrenheit, Length::Inch);

        let weather_url = OpenMeteo::new(coordinates)
        .current(vec![Current::Temperature, Current::IsDay, Current::WindSpeed])
        .units(units)
        .forecast_days(1)
        .build_url();

        let correct = String::from("latitude=20&longitude=50&timezone=auto&forecast_days=1&current=,temperature_2m,is_day,wind_speed_10m&wind_speed_unit=kn&temperature_unit=fahrenheit&precipitation_unit=inch");

        assert_eq!(weather_url, correct, "Incorrect url creation on OpenMeteo");
    }

    #[ignore = "This test should only be run when there is internet connection, and api.open-meteo.com is reachable"]
    #[tokio::test]
    async fn try_parsing() {
        let coordinates = Coordinates::new(50.0, 20.0);
        let units = Units::new(Speed::Knots, Temperature::Fahrenheit, Length::Inch);

        let options = OpenMeteo::new(coordinates)
        .current(vec![Current::Temperature, Current::IsDay, Current::WindSpeed])
        .units(units)
        .forecast_days(1);

        let result = options.parse().await;

        assert!(result.is_ok(), "Note: the test may fail if there is something with api.open-meteo.com, or your internet connection.");
    }

    #[tokio::test]
    async fn meowl() {
        let coordinates = Coordinates::new(50.0, 20.0);
        let units = Units::new(Speed::Knots, Temperature::Fahrenheit, Length::Inch);

        let options = OpenMeteo::new(coordinates)
        .hourly(vec![Hourly::Temperature, Hourly::IsDay, Hourly::WindSpeed])
        .units(units)
        .forecast_days(1);

        let result = options.parse().await;

        panic!("{:?}", result)
    }
}
