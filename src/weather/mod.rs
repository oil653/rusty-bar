/// The strucutres, shuch as CurrentWeather and HourlyWeather
mod structures;
pub use structures::{CurrentWeather, HourlyWeather};

/// Containes measurement primiteves such as speed, temperature or length
mod measurements;
pub use measurements::Units;

/// Contains the units for weather
mod units;

/// For parsing the weather information
mod parsing;