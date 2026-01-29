/// The strucutres, shuch as CurrentWeather and HourlyWeather
mod structures;
#[allow(unused_imports)]
pub use structures::{CurrentWeather, HourlyWeather};

/// Containes measurement primiteves such as speed, temperature or length
mod measurements;

/// Contains the units of weather, such as wind speed, cloud cover, weather code...
mod units;

/// For parsing the weather information
mod parsing;

#[allow(unused_imports)]
pub mod argument {
    pub use super::parsing::{ 
        Hourly, 
        Current, 
        PrecipitationType
    };
}

#[allow(unused_imports)]
pub mod prelude {
    use super::*;
    pub use measurements::{
        Units,
        Coordinates,
        Length,
        Speed,
        TempUnit
    };

    pub use parsing::{
        get_current,
        ParsingError
    };

    pub use super::argument;
}