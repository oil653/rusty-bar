use super::units::{
    Precipitation,
    WeatherCode,
    Wind,
    Temperature,
    Humidity
};
use super::measurements::Coordinates;

use chrono::{DateTime, TimeZone};

// The both return almost the same data, so it's fine to use one structure for both
pub type CurrentWeather<T> = HourlyWeather<T>;

pub struct HourlyWeather<T: TimeZone + Clone> {
    /// Coordinates of the weather data, this must be supplied
    coordinates: Coordinates,
    /// Time the data represents
    time: DateTime<T>,
    /// Temperature @ 2m, Celsius or Fahrenheit
    temperature: Option<Temperature>,
    /// Apparent, aka "feels like" temperature
    apparent_temperature: Option<Temperature>,
    /// Relative humidity %, 0 -> 100
    humidity: Option<Humidity>,
    /// True if it's daytime
    is_day: Option<bool>,
    precipitation: Option<Precipitation>,
    /// Weather code coresponds to the actual type of weather / weather events, such as rain, or sunshine
    /// It can be converted to a string or an emoji
    weather_code: Option<WeatherCode>,
    wind_speed: Option<Wind>
}

impl<T: TimeZone + Clone> HourlyWeather<T> {
    pub fn new(
        coordinates: Coordinates, 
        time: DateTime<T>, 
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
            weather_code, 
            wind_speed 
        }
    }
}