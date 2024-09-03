use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub hourly_units: HourlyUnits,
    pub hourly: Hourly,
}

#[derive(Deserialize, Debug)]
pub struct HourlyUnits {
    pub time: String,
    pub temperature_2m: String,
    pub rain: String,
    pub snowfall: String,
    pub weather_code: String,
}

#[derive(Deserialize, Debug)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f32>,
    pub rain: Vec<f32>,
    pub snowfall: Vec<f32>,
    pub weather_code: Vec<u8>,
}

pub fn gen_json(body: &String) -> Result<WeatherData> {
    let json: WeatherData = serde_json::from_str(&body).unwrap();
    Ok(json)
}