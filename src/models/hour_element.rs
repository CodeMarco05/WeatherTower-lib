use std::error::Error;

use chrono::prelude::*;

use super::raw_data::Hourly;

#[derive(Debug)]
pub struct Element{
    pub time: DateTime<Utc>,
    pub temperature: f32,
    pub rain: f32,
    pub snowfall: f32,
    pub weather_code: u8
}

impl Element {
    pub fn to_element_vec(hourly: &Hourly) -> Result<Vec<Element>, Box<dyn Error>>{
        let mut result: Vec<Element> = Vec::new();
        let format = "%Y-%m-%dT%H:%M";

        for i in 0..hourly.time.len() {
            let time_str = hourly.time.get(i).ok_or("Missing timestamp.").unwrap();
            let naive_time = NaiveDateTime::parse_from_str(&time_str, &format).expect("Error during naive-time conversiion.");
            let time = Utc.from_utc_datetime(&naive_time); // Corrected method

            let temperature = hourly.temperature_2m[i] as f32;
            let rain = hourly.rain[i] as f32;
            let snowfall = hourly.snowfall[i] as f32;
            let weather_code = hourly.weather_code[i] as u8;

            let element = Element {
                time,
                temperature,
                rain,
                snowfall,
                weather_code,
            };

            result.push(element);
        }
        Ok(result)
    }
    
}