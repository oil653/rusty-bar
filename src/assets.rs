use std::{collections::HashMap, sync::OnceLock};

pub static ASSETS_WEATHER: OnceLock<HashMap<&str, HashMap<&str, &str>>> = OnceLock::new();

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
        ("drizzle", include_str!("assets/svgs/weather/night/drizzle.svg")),
        ("foggy", include_str!("assets/svgs/weather/night/foggy.svg")),
        ("rainy", include_str!("assets/svgs/weather/night/rainy.svg")),
        ("snowfall", include_str!("assets/svgs/weather/night/snowfall.svg")),
        ("thunderstorm", include_str!("assets/svgs/weather/night/thunderstorm.svg")),
    ]);

    let map: HashMap<&str, HashMap<&str, &str>> = HashMap::from([
        ("day", day),
        ("night", night)
    ]);

    ASSETS_WEATHER.set(map).expect("Failed to set ASSETS_WEATHER");
}