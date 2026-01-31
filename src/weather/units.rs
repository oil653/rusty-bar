use super::measurements::{
    Length,
    Speed,
    TempUnit as TempUnit
};

/// Cloud cover over an area
#[derive(Debug, Clone, PartialEq)]
pub enum CloudCover {
    MainlyClear,
    Partial,
    Overcast
}

/// Basic intensity of a weather event
#[derive(Debug, Clone, PartialEq)]
pub enum Intensity {
    Light,
    Moderate, 
    Heavy
}

/// Intensity for weather events with 2 states
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleIntensity {
    Light,
    Heavy
}

/// Wind speed and direction
#[derive(Clone, Debug)]
pub struct Wind {
    pub speed: Option<f32>,
    pub direction: Option<f32>,
    pub unit: Speed
}
impl Wind {
    pub fn new<T: Copy + Into<f32>>(speed: Option<T>, direction: Option<T>, unit: Speed) -> Option<Self> {
        if speed.is_none() && direction.is_none() {
            None
        } else {
            Some(Self { 
                speed: speed.map(Into::into),
                direction: direction.map(Into::into),
                unit 
            })
        }
    }

    pub fn stringify(&self) -> String {
        format!("{}, {}", self.direction_stringify(), self.speed_stringify())
    }

    pub fn speed_stringify(&self) -> String {
        match self.speed {
            Some(speed) => format!("{}{}", speed, self.unit.stringify()),
            None => return String::from("")
        }
    }

    /// Stringify the direciton of the wind to the names of the direction
    /// For example if self.direction is 10 this returns "N"
    pub fn direction_stringify(&self) -> String {
        match self.direction {
            Some(direction) => {
                let normalized = direction % 360.0;
                let normalized = if normalized < 0.0 { normalized + 360.0 } else { normalized };
                
                match normalized {
                    d if d >= 337.5 || d < 22.5 => "N".to_string(),
                    d if d < 67.5 => "NE".to_string(),
                    d if d < 112.5 => "E".to_string(),
                    d if d < 157.5 => "SE".to_string(),
                    d if d < 202.5 => "S".to_string(),
                    d if d < 247.5 => "SW".to_string(),
                    d if d < 292.5 => "W".to_string(),
                    _ => "NW".to_string(),
                }
            },
            None => String::from("")
        }
    }
}


/// Rain precipitation in MM or INCH
#[derive(Clone, Debug)]
pub struct Precipitation {
    combined: Option<f32>,
    rain: Option<f32>,
    showers: Option<f32>,
    snowfall: Option<f32>,
    /// Probability should not be set with Current weather
    probability: Option<u8>,
    unit: Length
}
impl Precipitation {
    pub fn new<F, U>(
        combined: Option<F>, 
        rain: Option<F>, 
        showers: Option<F>, 
        snowfall: Option<F>, 
        probability: Option<U>,
        unit: Length) -> Option<Precipitation> 
    where 
        F: Copy + Into<f32>,
        U: Copy + Into<u8>
    {
        if combined.is_none() && rain.is_none() && showers.is_none() && snowfall.is_none() && probability.is_none() {
            None
        } else {
            Some(Precipitation { 
                combined: combined.map(Into::into),
                rain: rain.map(Into::into),
                showers: showers.map(Into::into),
                snowfall: snowfall.map(Into::into),
                probability: probability.map(Into::into),
                unit 
            })
        }
    }

    pub fn combined_to_string(&self) -> String {
        match self.combined {
            Some(value) => format!("{}{}", value, self.unit.to_string()),
            None => String::from("??")
        }
    }
    
    pub fn rain_to_string(&self) -> String {
        match self.rain {
            Some(value) => format!("{}{}", value, self.unit.to_string()),
            None => String::from("??")
        }        
    }
    
    pub fn showers_to_string(&self) -> String {
        match self.showers {
            Some(value) => format!("{}{}", value, self.unit.to_string()),
            None => String::from("??")
        }
    }
    
    pub fn snowfall_to_string(&self) -> String {
        match self.snowfall {
            Some(value) => format!("{}{}", value, self.unit.to_string()),
            None => String::from("??")
        }
    }

    pub fn probability_to_string(&self) -> String {
        match self.probability {
            Some(value) => format!("{}{}", value, self.unit.to_string()),
            None => String::from("??")
        }
    }
}

/// Simple temperature with stringification
#[derive(Debug, Clone)]
pub struct Temperature { 
    pub temp: f32,
    unit: TempUnit 
}
impl Temperature {
    pub fn new<T: Copy + Into<f32>>(temp: T, unit: TempUnit) -> Self {
        let temp = temp.into();
        Self { temp, unit }
    }
    pub fn stringify(&self) -> String {
        format!("{}{}", self.temp, self.unit.stringify())
    }
}

#[derive(Debug, Clone)]
pub struct Humidity {
    pub percentage: u8
}
impl Humidity {
    /// Percentage should be an int between 0 and 100
    pub fn new<T: Copy + Into<u8>>(percentage: T) -> Self {
        let percentage = percentage.into();
        Self { percentage }
    }
    pub fn stringify(&self) -> String {
        format!("{}%", self.percentage)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeatherCode {
    Clear,
    Cloudy(CloudCover),
    Fog{is_rime_fog: bool},
    Drizzle(Intensity),
    FreezingDrizzle(SimpleIntensity),
    Rain(Intensity),
    FreezingRain(SimpleIntensity),
    SnowFall(Intensity),
    SnowGrains,
    RainShowers(Intensity),
    SnowShowers(SimpleIntensity),
    Thunderstorm,
    ThunderstormWithHail(SimpleIntensity)
}

use std::fmt::Display;

impl Display for WeatherCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl WeatherCode {
    /// Constructs a WeatherCode instance from a weather code, returns none if the weather code isnt supported
    /// List of supported weather code: 
    ///    <table>
    ///  <thead>
    ///    <tr>
    ///      <th>Code</th>
    ///      <th>Description</th>
    ///    </tr>
    ///  </thead>
    ///  <tbody>
    ///    <tr><td>0</td><td>Clear sky</td></tr>
    ///    <tr><td>1, 2, 3</td><td>Mainly clear, partly cloudy, and overcast</td></tr>
    ///    <tr><td>45, 48</td><td>Fog and depositing rime fog</td></tr>
    ///    <tr><td>51, 53, 55</td><td>Drizzle: Light, moderate, and dense intensity</td></tr>
    ///    <tr><td>56, 57</td><td>Freezing drizzle: Light and dense intensity</td></tr>
    ///    <tr><td>61, 63, 65</td><td>Rain: Slight, moderate, and heavy intensity</td></tr>
    ///    <tr><td>66, 67</td><td>Freezing rain: Light and heavy intensity</td></tr>
    ///    <tr><td>71, 73, 75</td><td>Snow fall: Slight, moderate, and heavy intensity</td></tr>
    ///    <tr><td>77</td><td>Snow grains</td></tr>
    ///    <tr><td>80, 81, 82</td><td>Rain showers: Slight, moderate, and violent</td></tr>
    ///    <tr><td>85, 86</td><td>Snow showers: Slight and heavy</td></tr>
    ///    <tr><td>95 </td><td>Thunderstorm: Slight or moderate</td></tr>
    ///    <tr><td>96, 99</td><td>Thunderstorm with slight and heavy hail</td></tr>
    ///  </tbody>
    /// </table>
    /// source: https://open-meteo.com/en/docs?hourly=&current=weather_code#weather_variable_documentation
    pub fn from_code<T>(code: T) -> Option<Self> 
    where   
        T: Into<u8> + Copy
    {
        match code.into() {
            0 => Some(Self::Clear),
            1 => Some(Self::Cloudy(CloudCover::MainlyClear)),
            2 => Some(Self::Cloudy(CloudCover::Partial)),
            3 => Some(Self::Cloudy(CloudCover::Overcast)),
            45 => Some(Self::Fog { is_rime_fog: false }),
            48 => Some(Self::Fog { is_rime_fog: true }),
            51 => Some(Self::Drizzle(Intensity::Light)),
            53 => Some(Self::Drizzle(Intensity::Moderate)),
            55 => Some(Self::Drizzle(Intensity::Heavy)),
            56 => Some(Self::FreezingDrizzle(SimpleIntensity::Light)),
            57 => Some(Self::FreezingDrizzle(SimpleIntensity::Heavy)),
            61 => Some(Self::Rain(Intensity::Light)),
            63 => Some(Self::Rain(Intensity::Moderate)),
            65 => Some(Self::Rain(Intensity::Heavy)),
            66 => Some(Self::FreezingRain(SimpleIntensity::Light)),
            67 => Some(Self::FreezingRain(SimpleIntensity::Heavy)),
            71 => Some(Self::SnowFall(Intensity::Light)),
            73 => Some(Self::SnowFall(Intensity::Moderate)),
            75 => Some(Self::SnowFall(Intensity::Heavy)),
            77 => Some(Self::SnowGrains),
            80 => Some(Self::RainShowers(Intensity::Light)),
            81 => Some(Self::RainShowers(Intensity::Moderate)),
            82 => Some(Self::RainShowers(Intensity::Heavy)),
            85 => Some(Self::SnowShowers(SimpleIntensity::Light)),
            86 => Some(Self::SnowShowers(SimpleIntensity::Heavy)),
            95 => Some(Self::Thunderstorm),
            96 => Some(Self::ThunderstormWithHail(SimpleIntensity::Light)),
            99 => Some(Self::ThunderstormWithHail(SimpleIntensity::Heavy)),
            _ => None
        }
    }

    /// Converts a weather code back to a human readable string
    pub fn to_string(&self) -> String {
        match self {
            Self::Clear => "Clear sky".to_string(),

            Self::Cloudy(cloud_cover) => match cloud_cover {
                CloudCover::MainlyClear => "Mainly clear".to_string(),
                CloudCover::Partial => "Partly cloudy".to_string(),
                CloudCover::Overcast => "Overcast".to_string(),
            },

            Self::Fog { is_rime_fog } => {
                if *is_rime_fog {
                    "Rime fog".to_string()
                } else {
                    "Fog".to_string()
                }
            }

            Self::Drizzle(intensity) => match intensity {
                Intensity::Light => "Light drizzle".to_string(),
                Intensity::Moderate => "Moderate drizzle".to_string(),
                Intensity::Heavy => "Dense drizzle".to_string(),
            },

            Self::FreezingDrizzle(intensity) => match intensity {
                SimpleIntensity::Light => "Light freezing drizzle".to_string(),
                SimpleIntensity::Heavy => "Dense freezing drizzle".to_string(),
            },

            Self::Rain(intensity) => match intensity {
                Intensity::Light => "Light rain".to_string(),
                Intensity::Moderate => "Moderate rain".to_string(),
                Intensity::Heavy => "Heavy rain".to_string(),
            },

            Self::FreezingRain(intensity) => match intensity {
                SimpleIntensity::Light => "Light freezing rain".to_string(),
                SimpleIntensity::Heavy => "Heavy freezing rain".to_string(),
            },

            Self::SnowFall(intensity) => match intensity {
                Intensity::Light => "Light snowfall".to_string(),
                Intensity::Moderate => "Moderate snowfall".to_string(),
                Intensity::Heavy => "Heavy snowfall".to_string(),
            },

            Self::SnowGrains => "Snow grains".to_string(),

            Self::RainShowers(intensity) => match intensity {
                Intensity::Light => "Light rain showers".to_string(),
                Intensity::Moderate => "Moderate rain showers".to_string(),
                Intensity::Heavy => "Violent rain showers".to_string(),
            },

            Self::SnowShowers(intensity) => match intensity {
                SimpleIntensity::Light => "Light snow showers".to_string(),
                SimpleIntensity::Heavy => "Heavy snow showers".to_string(),
            },

            Self::Thunderstorm => "Thunderstorm".to_string(),

            Self::ThunderstormWithHail(intensity) => match intensity {
                SimpleIntensity::Light => "Thunderstorm with slight hail".to_string(),
                SimpleIntensity::Heavy => "Thunderstorm with heavy hail".to_string(),
            },
        }
    }
    /// Converts a weather code back to a string containing a utf emoji representing the weather condition
    pub fn to_emoji(&self, is_night: bool) -> String {
        match self {
            Self::Clear => {
                if is_night {
                    "☀️".to_string()
                } else {
                    "🌙".to_string()
                }
            },

            Self::Cloudy(cloud_cover) => match cloud_cover {
                CloudCover::MainlyClear => "🌤️".to_string(),
                _ => "🌥️".to_string()
            },

            Self::Fog {is_rime_fog: _} => "🌫️".to_string(),

            Self::Drizzle(_) => "🌦️".to_string(),

            Self::Rain(_) | Self::FreezingRain(_) | Self::RainShowers(_) => "🌧️".to_string(),

            Self::SnowFall(_) | Self::SnowShowers(_) | Self::FreezingDrizzle(_) => "️🌨️".to_string(),

            Self::SnowGrains => "❄️".to_string(),

            Self::Thunderstorm | Self::ThunderstormWithHail(_) => "⛈️".to_string(),
        }
    }

    pub fn get_svg_name(&self) -> String {
        match self {
            Self::Clear => "clear",
            Self::Cloudy(_) => "cloudy",
            #[allow(unused_variables)]
            Self::Fog {is_rime_fog: bool} => "foggy",
            Self::Drizzle(_) | Self::FreezingDrizzle(_) => "drizzle",
            Self::Rain(_) | Self::RainShowers(_) | Self::FreezingRain(_) => "rainy",
            Self::SnowFall(_) | Self::SnowGrains | Self::SnowShowers(_) => "snowfall",
            Self::Thunderstorm | Self::ThunderstormWithHail(_) => "thunderstorm"
        }
        .to_string()
    }
}
