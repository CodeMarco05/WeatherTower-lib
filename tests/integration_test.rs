use weather_tower_lib::get_json_weather_data;
use weather_tower_lib::query_params::QueryParams;

#[test]
fn test_get_json_weather_data() {
	let params = QueryParams {
		latitude: 47.782,
		longitude: 9.6106,
		hourly: vec!["temperature_2m", "rain", "snowfall", "weather_code"],
		timezone: "Europe/Berlin",
		past_days: 2,
	};

	get_json_weather_data(params);
	// Add assertions or checks here if needed
}