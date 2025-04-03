// src/services/weather.rs
// 天気情報サービス

use reqwest::Client;
use tokio::runtime::Runtime;

use crate::models::weather::WeatherResponse;

// 天気データの取得
pub fn fetch_weather(
    runtime: &mut Runtime,
    client: &Client,
    api_key: &str,
    location: &str,
    weather_data: &mut Option<WeatherResponse>,
) {
    if weather_data.is_none() {
        let client = client.clone();
        let api_key = api_key.to_string();
        let location = location.to_string();

        runtime.spawn(async move {
            let url = format!(
                "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&lang=ja",
                location, api_key
            );

            match client.get(&url).send().await {
                Ok(response) => match response.json::<WeatherResponse>().await {
                    Ok(data) => Some(data),
                    Err(_) => None,
                },
                Err(_) => None,
            }
        });
    }
}
