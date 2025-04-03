// src/models/weather.rs
// 天気情報のモデル

use serde::Deserialize;

// 天気情報のレスポンス用構造体
#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub weather: Vec<Weather>,
    pub main: MainWeather,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct MainWeather {
    pub temp: f32,
    pub humidity: i32,
}
