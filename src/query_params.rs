
pub struct QueryParams {
    pub latitude: f64,
    pub longitude: f64,
    pub hourly: Vec<&'static str>,
    pub timezone: &'static str,
    pub past_days: i32,
}

impl QueryParams {
    pub fn to_query_string(&self, base_url: &str) -> String {
        let mut url: String = String::from(base_url);

        url.push_str("?");

        url.push_str(&format!("latitude={}&longitude={}", self.latitude, self.longitude));

        if !self.hourly.is_empty(){
            url.push_str("&hourly=");
            url.push_str(&self.hourly.join(","));
        }

        url.push_str(&format!("&timezone={}&past_days={}", self.timezone, self.past_days));

        return url;
    }
}