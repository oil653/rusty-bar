/// Arguments for data to be parsed
mod arguments;

use super::{
    Units,
    CurrentWeather,
    measurements::{
        Coordinates,
        Speed,
        Length, 
        Temperature
    },
};

use open_meteo_rs::{
    Client,
    forecast::{self, ForecastResultHourly}
};

use chrono::TimeZone;

use std::error::Error;

async fn get_current<T: TimeZone + Clone>(
    coordinates: Coordinates, 
    units: Units, 
    arguments: Vec<arguments::Current>
)  {//-> Result<Option<CurrentWeather<T>>, Box<dyn Error>> {
    let client = Client::new();
    let mut opts = forecast::Options::default();

    opts.location = open_meteo_rs::Location { 
        lat: coordinates.lat, 
        lng: coordinates.lng 
    };

    // Set no elevation, let open-meteo decide
    opts.elevation = Some(forecast::Elevation::Nan);

    opts.temperature_unit = Some({
        use Temperature::*;
            match units.temperature {
            Celsius => forecast::TemperatureUnit::Celsius,
            Fahrenheit => forecast::TemperatureUnit::Fahrenheit
        }
    });

    opts.cell_selection = Some(open_meteo_rs::forecast::CellSelection::Nearest);

    // Making the units speed here is cleaner than matching it
    opts.wind_speed_unit = Some(units.speed.to_string().as_str().try_into().unwrap());
    opts.precipitation_unit = Some(units.length.to_string().as_str().try_into().unwrap());

    opts.current.extend(arguments.iter().map(|arg| arg.to_string()));

    let res = client.forecast(opts).await.unwrap();

    // let current = match res.current {
    //     Some(current) => current,
    //     None => return Ok(None)
    // };

    let current = res.current.unwrap();


}


fn hourly_deserialize(hourly_data: ForecastResultHourly) {//-> Result<Option<CurrentWeather<T>>, Box<dyn Error>> {

}