#![allow(unused)]
/// Base measurements

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Speed {
    Kmh, 
    Ms,
    Mph,
    Knots
}
#[allow(dead_code)]
impl Speed {
    pub fn stringify(&self) -> String {
        match self {
            Speed::Kmh => "km/h".to_string(),
            Speed::Ms => "m/s".to_string(),
            Speed::Mph => "mp/h".to_string(),
            Speed::Knots => "kn".to_string()
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Speed::Kmh => "kmh".to_string(),
            Speed::Ms => "ms".to_string(),
            Speed::Mph => "mph".to_string(),
            Speed::Knots => "kn".to_string()
        }
    }
}
impl Default for Speed {
    fn default() -> Self {
        Self::Kmh
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum TempUnit {
    Celsius,
    Fahrenheit
}
impl TempUnit {
    pub fn stringify(&self) -> String {
        match self {
            TempUnit::Celsius => "°C".to_string(),
            TempUnit::Fahrenheit => "°F".to_string()
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Celsius => "celsius".to_string(),
            Self::Fahrenheit => "fahrenheit".to_string()
        }
    }
}
impl Default for TempUnit {
    fn default() -> Self {
        Self::Celsius
    }
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Length {
    Mm,
    Inch
}
impl Length {
    pub fn to_string(&self) -> String {
        use Length::*;
        match self {
            Inch => "inch".to_string(),
            Mm => "mm".to_string(),
        }
    }
}
impl Default for Length {
    fn default() -> Self {
        Self::Mm
    }
}

/// A collection of primitive types
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Units {
    pub speed: Speed,
    pub temperature: TempUnit,
    pub length: Length
}
impl Units {
    pub fn new(speed: Speed, temperature: TempUnit, length: Length) -> Self {
        Units { speed, temperature, length }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    pub lng: f64,
    pub lat: f64
}
impl Coordinates {
    pub fn new(lng: f64, lat: f64) -> Self {
        Self { lng, lat }
    }
}
