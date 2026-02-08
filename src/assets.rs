use std::{collections::HashMap, sync::OnceLock};

pub static ASSETS_WEATHER: OnceLock<HashMap<&str, HashMap<&str, &str>>> = OnceLock::new();

/// PANICS if ASSETS_WEATHER is not initialized
pub fn get_svg(module: &str, name: &str) -> &'static str {
    let error_msg = format!("Failed to get asset {name} from module {module} from memory");

    ASSETS_WEATHER
        .get()
        .expect(error_msg.as_str())
        .get(module)
        .expect(error_msg.as_str())
        .get(name)
        .expect(error_msg.as_str())
}

pub fn load_assets() {
    let day: HashMap<&str, &str> = HashMap::from([
        ("clear", include_str!("assets/svgs/weather/day/clear.svg")),
        ("cloudy", include_str!("assets/svgs/weather/day/cloudy.svg")),
        ("drizzle", include_str!("assets/svgs/weather/day/drizzle.svg")),
        ("foggy", include_str!("assets/svgs/weather/day/foggy.svg")),
        ("rainy", include_str!("assets/svgs/weather/day/rainy.svg")),
        ("snowfall", include_str!("assets/svgs/weather/day/snowfall.svg")),
        ("thunderstorm", include_str!("assets/svgs/weather/day/thunderstorm.svg")),
    ]);

    let night: HashMap<&str, &str> = HashMap::from([
        ("clear", include_str!("assets/svgs/weather/night/clear.svg")),
        ("cloudy", include_str!("assets/svgs/weather/night/cloudy.svg")),
        ("drizzle", include_str!("assets/svgs/weather/day/drizzle.svg")),
        ("foggy", include_str!("assets/svgs/weather/night/foggy.svg")),
        ("rainy", include_str!("assets/svgs/weather/day/rainy.svg")),
        ("snowfall", include_str!("assets/svgs/weather/day/snowfall.svg")),
        ("thunderstorm", include_str!("assets/svgs/weather/day/thunderstorm.svg")),
    ]);

    let weather: HashMap<&str, &str> = HashMap::from([
        ("droplet", include_str!("assets/svgs/weather/droplet.svg")),
        ("humidity", include_str!("assets/svgs/weather/humidity.svg")),
        ("wind", include_str!("assets/svgs/weather/wind.svg")),
        ("temperature", include_str!("assets/svgs/weather/temperature.svg")),
        ("apparent_temperature", include_str!("assets/svgs/weather/feels_like.svg"))
    ]);

    let prec: HashMap<&str, &str> = HashMap::from([
        ("combined", include_str!("assets/svgs/weather/prec/combined.svg")),
        ("rain", include_str!("assets/svgs/weather/prec/rain.svg")),
        ("showers", include_str!("assets/svgs/weather/prec/showers.svg")),
        ("snow", include_str!("assets/svgs/weather/prec/snow.svg"))
    ]);

    let commons: HashMap<&str, &str> = HashMap::from([
        ("refresh", include_str!("assets/svgs/refresh.svg")),
        ("back", include_str!("assets/svgs/back.svg"))
    ]);

    let map: HashMap<&str, HashMap<&str, &str>> = HashMap::from([
        ("weather", weather),
        ("day", day),
        ("night", night),
        ("prec", prec),
        ("commons", commons)
    ]);

    ASSETS_WEATHER.set(map).expect("Failed to set ASSETS_WEATHER");
}