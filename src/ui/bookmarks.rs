// src/ui/bookmarks.rs
// ブックマークセクション

use eframe::egui;

use crate::config::types::Bookmarks;

// OSごとのコマンド実行を関数化
fn open_url(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(&["/C", "start", url])
            .spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
}

// Desktop Applicationsセクションのレンダリング
fn render_desktop_bookmarks(ui: &mut egui::Ui, bookmarks: &Bookmarks) {
    ui.heading("Desktop Applications");
    for category in &bookmarks.desktop {
        ui.collapsing(&category.name, |ui| {
            for bookmark in &category.items {
                if ui.button(&bookmark.name).clicked() {
                    open_url(&bookmark.url);
                }
            }
        });
    }
}

// WEB Applicationsセクションのレンダリング
fn render_web_bookmarks(ui: &mut egui::Ui, bookmarks: &Bookmarks) {
    ui.heading("WEB Applications");
    for category in &bookmarks.web {
        ui.collapsing(&category.name, |ui| {
            for bookmark in &category.items {
                if ui.button(&bookmark.name).clicked() {
                    open_url(&bookmark.url);
                }
            }
        });
    }
}

// ブックマークセクションのレンダリング
pub fn render(ui: &mut egui::Ui, bookmarks: &Bookmarks) {
    ui.add_space(20.0);

    egui::Grid::new("bookmarks_grid")
        .num_columns(2)
        .spacing([20.0, 10.0]) // 列間のスペースを調整
        .min_col_width(300.0) // 各列の最小幅を設定
        .show(ui, |ui| {
            ui.vertical(|ui| {
                render_desktop_bookmarks(ui, bookmarks);
            });

            ui.end_row();

            ui.vertical(|ui| {
                render_web_bookmarks(ui, bookmarks);
            });
        });
}
