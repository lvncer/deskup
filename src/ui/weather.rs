// src/ui/weather.rs
// 天気セクション

use eframe::egui;

use crate::models::weather::WeatherResponse;

// 天気セクションのレンダリング
pub fn render(ui: &mut egui::Ui, weather_data: &Option<WeatherResponse>, location: &str) {
    ui.add_space(20.0);
    ui.heading("Weather");
    if let Some(weather) = weather_data {
        ui.horizontal(|ui| {
            let temp = weather.main.temp - 273.15; // ケルビンから摂氏に変換
            let desc = &weather.weather[0].description;
            ui.label(format!(
                "Weather in {}: {}, Temperature: {:.1}°C, Humidity: {}%",
                location, desc, temp, weather.main.humidity
            ));
        });
    } else {
        ui.label("Loading weather data...");
    }
}
