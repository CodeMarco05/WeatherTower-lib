

use reqwest::blocking::ClientBuilder;

mod query_params;

mod models{
    pub mod raw_data;
    pub mod hour_element;
}


pub fn get_json_weather_data(){
    let client = ClientBuilder::new().build().unwrap();

    let base_url = "https://api.open-meteo.com/v1/forecast";

    let params = query_params::QueryParams{
        latitude: 47.782,
        longitude: 9.6106,
        hourly: vec!["temperature_2m", "rain", "snowfall", "weather_code"],
        timezone: "Europe/Berlin",
        past_days: 2
    };

    let url = params.to_query_string(base_url);

    let response = client.get(url).send().unwrap(); 

    let body = response.text().unwrap();

    let result = models::raw_data::gen_json(&body).expect("Error during Json generation.");

    let mut elements = models::hour_element::Element::to_element_vec(&result.hourly).expect("No Vector with Elements returned.");

    elements.sort_by_key(|e| e.time);

}