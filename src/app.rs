use eframe::{
    egui::{self},
    epi,
};
use reqwest::Client;
use std::env;
use std::fs;
use std::path::Path;
use tokio::runtime::Runtime;

use crate::config::types::AppConfig;
use crate::models::anniversary::JapaneseAnniversary;
use crate::models::holiday::HolidayResponse;
use crate::models::weather::WeatherResponse;

// メインアプリケーション構造体
pub struct MyApp {
    pub config: AppConfig,
    pub weather_data: Option<WeatherResponse>,
    pub joke: Option<String>,
    pub jp_anniversary: Option<Vec<JapaneseAnniversary>>,
    pub world_holiday: Option<Vec<HolidayResponse>>,
    pub client: Client,
    pub runtime: Runtime,
}

impl Default for MyApp {
    fn default() -> Self {
        // 設定ファイルのパスを取得
        let config_path = Path::new("config.toml");

        // ユーザー名の初期値をデスクトップ名にする
        let default_user_name = env::var("USER")
            .unwrap_or_else(|_| env::var("USERNAME").unwrap_or_else(|_| "ユーザー".to_string()));

        // 設定ファイルがなければデフォルト設定で作成
        let config = if !config_path.exists() {
            let default_config = AppConfig::default_with_username(default_user_name);

            // 設定ファイルを保存
            let toml_string = toml::to_string(&default_config).unwrap();
            fs::write(config_path, toml_string).unwrap();

            default_config
        } else {
            // 設定ファイルが存在する場合は読み込む
            let toml_str = fs::read_to_string(config_path).unwrap();
            toml::from_str(&toml_str).unwrap()
        };

        // HTTPクライアントとランタイムを初期化
        let client = Client::new();
        let runtime = Runtime::new().unwrap();

        MyApp {
            config,
            weather_data: None,
            joke: None,
            jp_anniversary: None,
            world_holiday: None,
            client,
            runtime,
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "マイデスクトップダッシュボード"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        // フォントを設定
        self.setup_fonts(ctx);

        // データの更新を行う
        self.update_data();

        // UIを構築
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DeskUp");

            // 挨拶セクション
            crate::ui::header::render(ui, &self.config.user_name);

            // 天気セクション
            crate::ui::weather::render(ui, &self.weather_data, &self.config.location);

            // 今日は何の日セクション
            crate::ui::anniversary::render(ui, &self.jp_anniversary, &self.world_holiday);

            // 今日の格言セクション
            ui.add_space(20.0);
            ui.heading("Joke of the day");
            if let Some(joke) = &self.joke {
                ui.label(joke);
            } else {
                ui.label("Loading quote data...");
            }

            // ブックマークセクション
            crate::ui::bookmarks::render(ui, &self.config.bookmarks);

            // TODOセクション
            crate::ui::todos::render(ui);
        });
    }
}

impl MyApp {
    // フォントを設定する関数
    fn setup_fonts(&self, ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // カスタムフォントを追加
        fonts.font_data.insert(
            "Inter".to_owned(),
            egui::FontData::from_static(include_bytes!("./fonts/Inter-VariableFont_opsz,wght.ttf")),
        );

        // フォントファミリーにカスタムフォントを設定
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "Inter".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "Inter".to_owned());

        // フォント設定を適用
        ctx.set_fonts(fonts);
    }

    // データ更新用メソッド
    pub fn update_data(&mut self) {
        // 各サービスからデータを取得
        crate::services::weather::fetch_weather(
            &mut self.runtime,
            &self.client,
            &self.config.weather_api_key,
            &self.config.location,
            &mut self.weather_data,
        );

        crate::services::joke::fetch_joke(&mut self.runtime, &self.client, &mut self.joke);

        crate::services::anniversary::fetch_anniversary(
            &mut self.runtime,
            &self.client,
            &mut self.jp_anniversary,
        );

        crate::services::holiday::fetch_holiday(
            &mut self.runtime,
            &self.client,
            &self.config.country_code,
            &mut self.world_holiday,
        );
    }
}
