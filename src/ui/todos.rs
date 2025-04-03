use eframe::egui;

// Google Tasksへのリンクを表示する関数
pub fn render(ui: &mut egui::Ui) {
    ui.add_space(20.0);
    ui.heading("Google Tasks");

    if ui.hyperlink("Open Google Tasks").clicked() {
        // Google TasksのURLを開く
        open_url("https://tasks.google.com/");
    }
}

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
        let _ = std::process::Command::new("xdg-open")
            .arg(url)
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg(url)
            .spawn();
    }
}
