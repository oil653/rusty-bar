/// Arguments for data to be parsed
mod arguments;
#[allow(unused_imports)]
pub use arguments::{
    Current,
    Hourly
};

use crate::weather::parsing::{arguments::{Argument}, open_meteo::OpenMeteo};

use super::{
    prelude::*,
    CurrentWeather,
    measurements::{
        Coordinates
    },
};

mod open_meteo;

use serde_json::Map;
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};

use ParsingError::MissingField;

use serde_json::Value;
use thiserror::Error;
use public_ip_address::{error::Error as WeatherError, perform_lookup};

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("Error with the client: {0}")]
    HTTP(reqwest::Error),
    #[error("Failed to deserialize returned data")]
    DeseializationError(serde_json::Error),
    #[error("Missing required field in API response: {0}")]
    MissingField(String),
    #[error("Error with time operation: {0}")]
    TimeError(String),
    #[error("Error with getting the current location {0}")]
    LocationError(WeatherError),
    #[error("Coordinate missing")]
    CoordinateMissing
}

async fn get_location() -> Result<Coordinates, ParsingError> {
    match perform_lookup(None).await {
        Ok(v) => {
            let lng = v.longitude;
            let lat = v.latitude;
            if lng.is_none() | lat.is_none() {
                Err(ParsingError::CoordinateMissing)
            } else {
                Ok(Coordinates::new(lng.unwrap(), lat.unwrap()))
            }
        },
        Err(e) => Err(ParsingError::LocationError(e))
    }
}

use std::error::Error;
pub async fn get_current<T: TimeZone + Clone>(
    coordinates: Option<Coordinates>, 
    units: Units, 
    arguments: Vec<arguments::Current>
) -> Result<CurrentWeather, Box<dyn Error>> {
    
    let coordinates = match coordinates{
        Some(v) => v,
        None => {
            get_location().await?
        }
    };

    let client = OpenMeteo::new(coordinates)
    .units(units.clone())
    .current(arguments.clone());

    let result = client.parse().await.unwrap();     // REPLACE .unwrap with ?

    let current = result["current"].as_object().ok_or(MissingField(String::from("current")))?;

    
    let coordinates = Coordinates::new(
        result["longitude"]
            .as_f64()
            .ok_or(MissingField(String::from("longtitude")))?, 
        result["latitude"]
            .as_f64()
            .ok_or(MissingField(String::from("latitude")))?
    );


    let utc_time = current["time"]
        .as_str()
        .ok_or(MissingField(String::from("current.time")))?;
    let utc_offset = result["utc_offset_seconds"]
        .as_i64()
        .ok_or(MissingField(String::from("utc_offset_seconds")))?;

    let time = convert_date_time(utc_time, utc_offset as i32)?;


    let temp = arguments
        .contains(&Current::Temperature)
        .then(|| parse_as_u64(current, Current::Temperature))
        .transpose()?;

    let app_temp = arguments
        .contains(&Current::ApparentTemp)
        .then(|| parse_as_f64(current, Current::ApparentTemp))
        .transpose()?;

    let humidity = arguments
        .contains(&Current::Humidity)
        .then(|| parse_as_u64(current, Current::Humidity))
        .transpose()?;

    let is_day = arguments
        .contains(&Current::IsDay)
        .then(|| parse_as_bool(current, Current::IsDay))
        .transpose()?;

    let prec = arguments
        .contains(&Current::Precipitation(arguments::PrecipitationTypes::Combined))
        .then(|| parse_as_f64(current, Current::Precipitation(arguments::PrecipitationTypes::Combined)))
        .transpose()?;

    let rain = arguments
        .contains(&Current::Precipitation(arguments::PrecipitationTypes::Rain))
        .then(|| parse_as_f64(current, Current::Precipitation(arguments::PrecipitationTypes::Rain)))
        .transpose()?;

    let showers = arguments
        .contains(&Current::Precipitation(arguments::PrecipitationTypes::Showers))
        .then(|| parse_as_f64(current, Current::Precipitation(arguments::PrecipitationTypes::Showers)))
        .transpose()?;

    let snowfall = arguments
        .contains(&Current::Precipitation(arguments::PrecipitationTypes::Snowfall))
        .then(|| parse_as_f64(current, Current::Precipitation(arguments::PrecipitationTypes::Snowfall)))
        .transpose()?;

    // For current weather there is no probability
    let probability = None;

    let weather_code = arguments
        .contains(&Current::WeatherCode)
        .then(|| parse_as_u64(current, Current::WeatherCode))
        .transpose()?;

    let wind_speed = arguments
        .contains(&Current::WindSpeed)
        .then(|| parse_as_f64(current, Current::WindSpeed))
        .transpose()?;

    let wind_dir = arguments
        .contains(&Current::WindDirection)
        .then(|| parse_as_f64(current, Current::WindDirection))
        .transpose()?;

    Ok(CurrentWeather::new_short(
            units,
            coordinates,
            time,
            temp.map(|v| v as f32), 
            app_temp.map(|v| v as f32), 
            humidity.map(|h: u64| h as u8), 
            is_day, 
            prec.map(|v| v as f32), 
            rain.map(|v| v as f32), 
            showers.map(|v| v as f32), 
            snowfall.map(|v| v as f32), 
            probability.map(|h: u64| h as u8), 
            weather_code.map(|h: u64| h as u8), 
            wind_speed.map(|v| v as f32), 
            wind_dir.map(|v| v as f32)
        )
    )
}

fn parse_as_f64<A: Argument>(parse_in: &Map<String, Value>, field_name: A) -> Result<f64, ParsingError>
{
    parse_in[field_name.to_string().as_str()]
        .as_f64()
        .ok_or(MissingField(String::from(field_name.to_string())))
}

fn parse_as_u64<A: Argument>(parse_in: &Map<String, Value>, field_name: A) -> Result<u64, ParsingError>
{
    parse_in[field_name.to_string().as_str()]
        .as_u64()
        .ok_or(MissingField(String::from(field_name.to_string())))
}

fn parse_as_bool<A: Argument>(parse_in: &Map<String, Value>, field_name: A) -> Result<bool, ParsingError>
{
    parse_in[field_name.to_string().as_str()]
        .as_bool()
        .ok_or(MissingField(String::from(field_name.to_string())))
}

fn convert_date_time(iso8601: &str, offset: i32) -> Result<DateTime<FixedOffset>, ParsingError> {
    let naive_time = match NaiveDateTime::parse_from_str(iso8601, "%Y-%m-%dT%H:%M"){
        Ok(time) => time,
        Err(e) => return Err(ParsingError::TimeError(format!("Failed to parse iso8601 from '{iso8601}': {e}")))
    };

    let offset = {
        let option = if offset < 0 {
                FixedOffset::west_opt(offset.abs() as i32)
            } else {
                FixedOffset::east_opt(offset.abs() as i32)
            };
        match option {
            Some(offset) => offset,
            None => return Err(ParsingError::TimeError(format!("Failed to parse offset from '{offset}'")))
        }
    };

    let time: DateTime<FixedOffset> = DateTime::from_naive_utc_and_offset(naive_time, offset);

    Ok(time)
}