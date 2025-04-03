// src/ui/header.rs
// ヘッダーセクション（挨拶など）

use chrono::{Local, Timelike};
use eframe::egui;

// ヘッダーセクションのレンダリング
pub fn render(ui: &mut egui::Ui, user_name: &str) {
    ui.horizontal(|ui| {
        // 現在時刻から適切な挨拶を選択
        let now = Local::now();
        let hour = now.hour();

        let greeting = if hour < 12 {
            "Good morning!"
        } else if hour < 18 {
            "Hello!"
        } else {
            "Good evening!"
        };

        ui.heading(format!("{}, {}. Is the coffee ready?", greeting, user_name));
    });
}
