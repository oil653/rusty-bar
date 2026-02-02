/// Arguments for data to be parsed
mod arguments;
#[allow(unused_imports)]
pub use arguments::{
    Current,
    Hourly,
    PrecipitationType
};

mod open_meteo;


use crate::weather::{HourlyWeather, parsing::{
    arguments::Argument,
    open_meteo::OpenMeteo
}};

use super::{
    CurrentWeather,
    measurements::{
        Coordinates,
        Units
    }
};

use serde_json::Map;
use chrono::{
    DateTime, 
    FixedOffset, 
    NaiveDateTime
};

use serde_json::Value;
use thiserror::Error;
use public_ip_address::perform_lookup;

use ParsingError::MissingField;

#[derive(Debug, Error, Clone)]
pub enum ParsingError {
    #[error("Error with the client: {0}")]
    HTTP(String),
    #[error("Failed to deserialize returned data")]
    DeseializationError(String),
    #[error("Missing required field in API response: {0}")]
    MissingField(String),
    #[error("Error with time operation: {0}")]
    TimeError(String),
    #[error("Error with getting the current location {0}")]
    LocationError(String),
    #[error("Unkown error: {0}")]
    OtherError(String)
}

/// Gets the current location of the device by ip
async fn get_location() -> Result<Coordinates, ParsingError> {
    match perform_lookup(None).await {
        Ok(response) => {
            match (response.longitude, response.latitude) {
                (Some(lng), Some(lat)) => Ok(Coordinates::new(lng, lat)),
                _ => Err(ParsingError::LocationError("Coordinates missing from perform_lookup answer".to_string()))
            }
        },
        Err(e) => Err(ParsingError::LocationError(e.to_string()))
    }
}

/// Yields the underlying coordinates, or performs a lookup
async fn get_coordinates(coordinates: Option<Coordinates>) -> Result<Coordinates, ParsingError> {
    match coordinates{
        Some(v) => Ok(v),
        None => {
            Ok(get_location().await?)
        }
    }
}

async fn perform_request(client: OpenMeteo) -> Result<Value, ParsingError> {
    match client.parse().await {
        Ok(value) => Ok(value),
        Err(e) => {
            return if let Some(e) = e.downcast_ref::<serde_json::error::Error>() {
                Err(ParsingError::DeseializationError(e.to_string()))
            } else if let Some(e) = e.downcast_ref::<reqwest::Error>() {
                Err(ParsingError::HTTP(e.to_string()))
            } else {
                Err(ParsingError::OtherError(e.to_string()))
            }
        }
    }
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

pub async fn get_current(
    coordinates: Option<Coordinates>,
    units: Units,
    arguments: Vec<arguments::Current>,
) -> Result<CurrentWeather, ParsingError> {
    let client = OpenMeteo::new(get_coordinates(coordinates).await?)
    .units(units.clone())
    .current(arguments.clone());

    let result = perform_request(client).await?;
    
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
        .then(|| parse_as_f64(current, Current::Temperature))
        .transpose()?;

    let app_temp = arguments
        .contains(&Current::ApparentTemp)
        .then(|| parse_as_f64(current, Current::ApparentTemp))
        .transpose()?;

    let humidity = arguments
        .contains(&Current::Humidity)
        .then(|| parse_as_u64(current, Current::Humidity))
        .transpose()?;

    // Please someone tell open-meteo that json can take bools, it doesnt have to be an int....
    let is_day = arguments
        .contains(&Current::IsDay)
        .then(|| parse_as_u64(current, Current::IsDay))
        .transpose()?;

    let prec = arguments
        .contains(&Current::Precipitation(arguments::PrecipitationType::Combined))
        .then(|| parse_as_f64(current, Current::Precipitation(arguments::PrecipitationType::Combined)))
        .transpose()?;

    let rain = arguments
        .contains(&Current::Precipitation(arguments::PrecipitationType::Rain))
        .then(|| parse_as_f64(current, Current::Precipitation(arguments::PrecipitationType::Rain)))
        .transpose()?;

    let showers = arguments
        .contains(&Current::Precipitation(arguments::PrecipitationType::Showers))
        .then(|| parse_as_f64(current, Current::Precipitation(arguments::PrecipitationType::Showers)))
        .transpose()?;

    let snowfall = arguments
        .contains(&Current::Precipitation(arguments::PrecipitationType::Snowfall))
        .then(|| parse_as_f64(current, Current::Precipitation(arguments::PrecipitationType::Snowfall)))
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
            is_day.map(|is_day| is_day != 0),
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

pub async fn get_hourly(
    coordinates: Option<Coordinates>,
    units: Units, 
    arguments: impl IntoIterator<Item = arguments::Hourly>,
    forecast_hours: u8
) -> Result<Vec<HourlyWeather>, ParsingError> {
    let arguments: Vec<arguments::Hourly> = arguments.into_iter().collect();

    let client = OpenMeteo::new(get_coordinates(coordinates).await?)
        .units(units.clone())
        .hourly(arguments.clone());

    let result = perform_request(client).await?;

    let coordinates = Coordinates::new(
        result["longitude"]
            .as_f64()
            .ok_or(MissingField(String::from("longtitude")))?, 
        result["latitude"]
            .as_f64()
            .ok_or(MissingField(String::from("latitude")))?
    );

    let utc_offset = result["utc_offset_seconds"]
        .as_i64()
        .ok_or(MissingField(String::from("utc_offset_seconds")))?;

    let hourly = result["hourly"].as_object().ok_or(MissingField(String::from("hourly")))?;

    let mut hours = Vec::new();
    for (id, hour) in hourly["time"].as_array().ok_or(MissingField(String::from("time")))?.iter().enumerate() {
        let time = convert_date_time(
                hour.as_str().ok_or(MissingField(String::from("hour.time.time")))?, 
                utc_offset as i32
        )?;

        let temp = arguments
            .contains(&Hourly::Temperature)
            .then(|| id_array_f64(hourly, Hourly::Temperature, id))
            .transpose()?;

        let app_temp = arguments
            .contains(&Hourly::ApparentTemp)
            .then(|| id_array_f64(hourly, Hourly::ApparentTemp, id))
            .transpose()?;

        let humidity = arguments
            .contains(&Hourly::Humidity)
            .then(|| id_array_u64(hourly, Hourly::Humidity, id))
            .transpose()?;

        let is_day = arguments
            .contains(&Hourly::IsDay)
            .then(|| id_array_u64(hourly, Hourly::IsDay, id))
            .transpose()?;

        let prec = arguments
            .contains(&Hourly::Precipitation(arguments::PrecipitationType::Combined))
            .then(|| id_array_f64(
                hourly,
                Hourly::Precipitation(arguments::PrecipitationType::Combined),
                id,
            ))
            .transpose()?;

        let rain = arguments
            .contains(&Hourly::Precipitation(arguments::PrecipitationType::Rain))
            .then(|| id_array_f64(
                hourly,
                Hourly::Precipitation(arguments::PrecipitationType::Rain),
                id,
            ))
            .transpose()?;

        let showers = arguments
            .contains(&Hourly::Precipitation(arguments::PrecipitationType::Showers))
            .then(|| id_array_f64(
                hourly,
                Hourly::Precipitation(arguments::PrecipitationType::Showers),
                id,
            ))
            .transpose()?;

        let snowfall = arguments
            .contains(&Hourly::Precipitation(arguments::PrecipitationType::Snowfall))
            .then(|| id_array_f64(
                hourly,
                Hourly::Precipitation(arguments::PrecipitationType::Snowfall),
                id,
            ))
            .transpose()?;

        // For hourly weather there is no probability
        let probability = arguments
            .contains(&Hourly::PrecipitationProbability)
            .then(|| id_array_u64(
                hourly, 
                Hourly::PrecipitationProbability, 
                id)
            )
            .transpose()?;

        let weather_code = arguments
            .contains(&Hourly::WeatherCode)
            .then(|| id_array_u64(hourly, Hourly::WeatherCode, id))
            .transpose()?;

        let wind_speed = arguments
            .contains(&Hourly::WindSpeed)
            .then(|| id_array_f64(hourly, Hourly::WindSpeed, id))
            .transpose()?;

        let wind_dir = arguments
            .contains(&Hourly::WindDirection)
            .then(|| id_array_f64(hourly, Hourly::WindDirection, id))
            .transpose()?;

        hours.push(HourlyWeather::new_short(
                units.clone(),
                coordinates.clone(),
                time,
                temp.map(|v| v as f32), 
                app_temp.map(|v| v as f32), 
                humidity.map(|h: u64| h as u8), 
                is_day.map(|is_day| is_day != 0),
                prec.map(|v| v as f32), 
                rain.map(|v| v as f32), 
                showers.map(|v| v as f32), 
                snowfall.map(|v| v as f32), 
                probability.map(|h: u64| h as u8), 
                weather_code.map(|h: u64| h as u8), 
                wind_speed.map(|v| v as f32), 
                wind_dir.map(|v| v as f32)
        ));
    };

    Ok(hours)
}

fn id_array_f64<A: Argument>(parse_in: &Map<String, Value>, field_name: A, id: usize) -> Result<f64, ParsingError>
{
    parse_in[field_name.to_string().as_str()]
        .as_array()
        .ok_or(MissingField(String::from(field_name.to_string())))?
        [id]
        .clone()
        .as_f64()
        .ok_or(ParsingError::DeseializationError(format!("Failed to parse x.{}[{}] as f64", field_name.to_string(), id)))
}

fn id_array_u64<A: Argument>(parse_in: &Map<String, Value>, field_name: A, id: usize) -> Result<u64, ParsingError>
{
    parse_in[field_name.to_string().as_str()]
        .as_array()
        .ok_or(MissingField(String::from(field_name.to_string())))?
        [id]
        .clone()
        .as_u64()
        .ok_or(ParsingError::DeseializationError(format!("Failed to parse x.{}[{}] as u64", field_name.to_string(), id)))
}