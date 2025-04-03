// src/ui/anniversary.rs
// 記念日セクション

use eframe::egui;

use crate::models::anniversary::JapaneseAnniversary;
use crate::models::holiday::HolidayResponse;

// 記念日セクションのレンダリング
pub fn render(
    ui: &mut egui::Ui,
    jp_anniversary: &Option<Vec<JapaneseAnniversary>>,
    world_holiday: &Option<Vec<HolidayResponse>>,
) {
    ui.add_space(20.0);
    ui.heading("What day is it today?");
    ui.collapsing("Anniversary in Japan", |ui| {
        if let Some(anniversaries) = jp_anniversary {
            for anniv in anniversaries {
                ui.label(format!("・{}: {}", anniv.name, anniv.description));
            }
        } else {
            ui.label("Loading anniversary data...");
        }
    });

    ui.collapsing("Anniversary in the world", |ui| {
        if let Some(holidays) = world_holiday {
            for holiday in holidays {
                ui.label(format!(
                    "・{}: {} ({})",
                    holiday.date, holiday.name, holiday.local_name
                ));
            }
        } else {
            ui.label("Loading anniversary data...");
        }
    });
}
