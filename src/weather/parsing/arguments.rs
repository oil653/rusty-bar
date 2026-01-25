/// All open-meteo-argument types should implement this trait
#[allow(unused)]
pub trait Argument {
    /// should return the corresponding [open-meteo option](https://open-meteo.com/en/docs#api_documentation)
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub enum PrecipitationTypes {
    Combined,
    Rain,
    Showers,
    Snowfall,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub enum Current {
    Temperature,
    ApparentTemp,
    Humidity,
    IsDay,
    Precipitation(PrecipitationTypes),
    WeatherCode,
    WindSpeed,
    WindDirection
}
impl Argument for Current {
    fn to_string(&self) -> String {
        use Current::*;
        match self {
            Temperature => String::from("temperature_2m"),
            ApparentTemp => String::from("apparent_temperature"),
            Humidity => String::from("relative_humidity_2m"),
            IsDay => String::from("is_day"),
            WeatherCode => String::from("weather_code"),
            WindSpeed => String::from("wind_speed_10m"),
            WindDirection => String::from("wind_direction_10m"),
            Precipitation(precipitation) => {
                use PrecipitationTypes::*;
                match precipitation {
                    Combined => String::from("precipitation"),
                    Rain => String::from("rain"),
                    Showers => String::from("showers"),                    
                    Snowfall => String::from("snowfall"),
                    
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub enum Hourly {
    Temperature,
    ApparentTemp,
    Humidity,
    IsDay,
    Precipitation(PrecipitationTypes),
    PrecipitationProbability,
    WeatherCode,
    WindSpeed,
    WindDirection
}
impl Argument for Hourly {
    fn to_string(&self) -> String {
        use Hourly::*;
        match self {
            Temperature => String::from("temperature_2m"),
            ApparentTemp => String::from("apparent_temperature"),
            Humidity => String::from("relative_humidity_2m"),
            IsDay => String::from("is_day"),
            WeatherCode => String::from("weather_code"),
            WindSpeed => String::from("wind_speed_10m"),
            WindDirection => String::from("wind_direction_10m"),
            PrecipitationProbability => String::from("precipitation_probability"),
            Precipitation(precipitation) => {
                use PrecipitationTypes::*;
                match precipitation {
                    Combined => String::from("precipitation"),
                    Rain => String::from("rain"),
                    Showers => String::from("showers"),                    
                    Snowfall => String::from("snowfall"),
                    
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_arguments_correct() {
        let args: Vec<String> = vec![Current::Temperature, Current::IsDay, Current::Precipitation(PrecipitationTypes::Combined)]
        .iter()
        .map(|arg| arg.to_string())
        .collect();
        
        let manual_args: Vec<String> = vec!["temperature_2m", "is_day", "precipitation"]
        .iter()
        .map(|arg| arg.to_string())
        .collect();

        assert!(args == manual_args)
    }

    #[test]
    fn hourly_arguments_correct() {
        let args: Vec<String> = vec![Hourly::Temperature, Hourly::IsDay, Hourly::Precipitation(PrecipitationTypes::Combined)]
        .iter()
        .map(|arg| arg.to_string())
        .collect();
        
        let manual_args: Vec<String> = vec!["temperature_2m", "is_day", "precipitation"]
        .iter()
        .map(|arg| arg.to_string())
        .collect();

        assert!(args == manual_args)
    }
}