/// Base measurements

#[derive(Clone, Debug)]
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


#[derive(Clone, Debug)]
pub enum Temperature {
    Celsius,
    Fahrenheit
}
impl Temperature {
    pub fn stringify(&self) -> String {
        match self {
            Temperature::Celsius => "°C".to_string(),
            Temperature::Fahrenheit => "°F".to_string()
        }
    }
}


#[derive(Clone, Debug)]
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

/// A collection of primitive types
#[derive(Clone, Debug)]
pub struct Units {
    pub speed: Speed,
    pub temperature: Temperature,
    pub length: Length
}
impl Units {
    pub fn new(speed: Speed, temperature: Temperature, length: Length) -> Self {
        Units { speed, temperature, length }
    }
}

#[derive(Debug, Clone)]
pub struct Coordinates {
    pub lng: f64,
    pub lat: f64
}
impl Coordinates {
    pub fn new(lng: f64, lat: f64) -> Self {
        Self { lng, lat }
    }
}
