use super::units::{
    Precipitation,
    WeatherCode,
    Wind,
    Temperature,
    Humidity,
};
use super::measurements::{Coordinates, Units};

use chrono::{DateTime, FixedOffset};

// The both return almost the same data, so it's fine to use one structure for both
pub type CurrentWeather = HourlyWeather;

#[derive(Debug, Clone)]
pub struct HourlyWeather {
    /// Coordinates of the weather data, this must be supplied
    pub coordinates: Coordinates,
    /// Time the data represents
    pub time: DateTime<FixedOffset>,
    /// Temperature @ 2m, Celsius or Fahrenheit
    pub temperature: Option<Temperature>,
    /// Apparent, aka "feels like" temperature
    pub apparent_temperature: Option<Temperature>,
    /// Relative humidity %, 0 -> 100
    pub humidity: Option<Humidity>,
    /// True if it's daytime
    pub is_day: Option<bool>,
    pub precipitation: Option<Precipitation>,
    /// Weather code coresponds to the actual type of weather / weather events, such as rain, or sunshine
    /// It can be converted to a string or an emoji
    pub code: Option<WeatherCode>,
    pub wind: Option<Wind>
}

impl HourlyWeather {
    pub fn new(
        coordinates: Coordinates, 
        time: DateTime<FixedOffset>,
        temperature: Option<Temperature>, 
        apparent_temperature: Option<Temperature>,
        humidity: Option<Humidity>,
        is_day: Option<bool>,
        precipitation: Option<Precipitation>,
        weather_code: Option<WeatherCode>,
        wind_speed: Option<Wind>
    ) -> Self {
        Self { 
            coordinates, 
            time, 
            temperature, 
            apparent_temperature,
            humidity, 
            is_day, 
            precipitation, 
            code: weather_code, 
            wind: wind_speed 
        }
    }

    pub fn new_short<F, US>(
        units: Units, 
        coordinates: Coordinates, 
        time: DateTime<FixedOffset>,
        temp: Option<F>,
        apparent_temp: Option<F>,
        humidity: Option<US>,
        is_day: Option<bool>,
        prec: Option<F>,
        rain: Option<F>,
        showers: Option<F>,
        snowfall: Option<F>,
        probability: Option<US>,
        weather_code: Option<US>,
        wind_speed: Option<F>,
        wind_dir: Option<F>
    ) -> Self 
    where 
        F: Copy + Into<f32>,
        US: Copy + Into<u8>,
    {
        let temp = temp.map(|temp| Temperature::new(temp, units.temperature.clone()));
        let apparent_temp = apparent_temp.map(|apparent_temp| Temperature::new(apparent_temp, units.temperature));

        let humidity = humidity.map(Humidity::new);

        let precipitation = Precipitation::new(prec, rain, showers, snowfall, probability, units.length);

        let weather_code: Option<WeatherCode> = weather_code.and_then(WeatherCode::from_code);

        let wind = Wind::new(wind_speed, wind_dir, units.speed);

        Self::new(
            coordinates, 
            time, 
            temp, 
            apparent_temp, 
            humidity, 
            is_day, 
            precipitation, 
            weather_code, 
            wind
        )
    }
}