use super::units::{
    Precipitation,
    WeatherCode,
    Wind
};

pub struct CurrentWeather {
    /// Temperature @ 2m, Celsius or Fahrenheit
    temperature: Option<f32>,
    /// Apparent, aka "feels like" temperature
    apparent_temperature: Option<f32>,
    /// Relative humidity %, 0 -> 100
    humidity: Option<u8>,
    /// True if it's daytime
    is_day: Option<bool>,
    precipitation: Option<Precipitation>,
    /// Weather code coresponds to the actual type of weather / weather events, such as rain, or sunshine
    /// It can be converted to a string or an emoji
    weather_code: Option<WeatherCode>,
    wind_speed: Option<Wind>
}