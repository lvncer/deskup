use chrono::{DateTime, Datelike, Local, Timelike}; // Datelike と Timelike トレイトを追加
use eframe::{egui, epi};
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use tokio::runtime::Runtime;
use toml;

// ユーザー設定を格納する構造体
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AppConfig {
    user_name: String,
    weather_api_key: String,
    location: String,
    country_code: String,
    bookmarks: Bookmarks,
    notion_api_key: Option<String>,
    notion_database_id: Option<String>,
}

// ブックマークを格納する構造体
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Bookmarks {
    desktop: Vec<BookmarkCategory>,
    web: Vec<BookmarkCategory>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct BookmarkCategory {
    name: String,
    items: Vec<Bookmark>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Bookmark {
    name: String,
    url: String,
}

// 天気情報のレスポンス用構造体
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: MainWeather,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
struct MainWeather {
    temp: f32,
    humidity: i32,
}

// 格言API用の構造体
#[derive(Deserialize, Debug)]
struct JokeResponse {
    setup: String,
    punchline: String,
}

// 日本の記念日API用の構造体
#[derive(Deserialize, Debug)]
struct JapaneseAnniversaryResponse {
    anniversaries: Vec<JapaneseAnniversary>,
}

#[derive(Deserialize, Debug)]
struct JapaneseAnniversary {
    name: String,
    description: String,
}

// 世界の祝日API用の構造体
#[derive(Deserialize, Debug)]
struct HolidayResponse {
    date: String,
    name: String,
    #[serde(rename = "localName")]
    local_name: String,
}

// Notionタスク用の構造体
#[derive(Deserialize, Debug)]
struct NotionTodoResponse {
    results: Vec<NotionPage>,
}

#[derive(Deserialize, Debug)]
struct NotionPage {
    id: String,
    properties: NotionProperties,
}

#[derive(Deserialize, Debug)]
struct NotionProperties {
    title: NotionTitle,
    status: Option<NotionStatus>,
}

#[derive(Deserialize, Debug)]
struct NotionTitle {
    title: Vec<NotionText>,
}

#[derive(Deserialize, Debug)]
struct NotionText {
    plain_text: String,
}

#[derive(Deserialize, Debug)]
struct NotionStatus {
    select: Option<NotionSelect>,
}

#[derive(Deserialize, Debug)]
struct NotionSelect {
    name: String,
}

// メインアプリケーション構造体
struct MyApp {
    config: AppConfig,
    weather_data: Option<WeatherResponse>,
    joke: Option<String>,
    jp_anniversary: Option<Vec<JapaneseAnniversary>>,
    world_holiday: Option<Vec<HolidayResponse>>,
    todos: Option<Vec<(String, String)>>,
    client: Client,
    runtime: Runtime,
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
            let default_config = AppConfig {
                user_name: default_user_name,
                weather_api_key: "your_api_key_here".to_string(),
                location: "Tokyo".to_string(),
                country_code: "JP".to_string(),
                bookmarks: Bookmarks {
                    desktop: vec![BookmarkCategory {
                        name: "作業".to_string(),
                        items: vec![Bookmark {
                            name: "メモ帳".to_string(),
                            url: "notepad".to_string(),
                        }],
                    }],
                    web: vec![BookmarkCategory {
                        name: "検索".to_string(),
                        items: vec![Bookmark {
                            name: "Google".to_string(),
                            url: "https://google.com".to_string(),
                        }],
                    }],
                },
                notion_api_key: None,
                notion_database_id: None,
            };

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
            todos: None,
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
        // データの更新を行う
        self.update_data();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DeskUp");

            // 挨拶セクション
            ui.add_space(10.0);
            ui.heading("👋");
            ui.horizontal(|ui| {
                // 現在時刻から適切な挨拶を選択
                let now: DateTime<Local> = Local::now();
                let hour = now.hour();

                let greeting = if hour < 12 {
                    "Good morning!"
                } else if hour < 18 {
                    "Hello!"
                } else {
                    "Good evening!"
                };

                ui.label(format!(
                    "{}, {}. Is the coffee ready?",
                    greeting, self.config.user_name
                ));
            });

            // 天気セクション
            ui.add_space(20.0);
            ui.heading("Weather");
            if let Some(weather) = &self.weather_data {
                ui.horizontal(|ui| {
                    let temp = weather.main.temp - 273.15; // ケルビンから摂氏に変換
                    let desc = &weather.weather[0].description;
                    ui.label(format!(
                        "Weather in {}: {}, Temperature: {:.1}°C, Humidity: {}%",
                        self.config.location, desc, temp, weather.main.humidity
                    ));
                });
            } else {
                ui.label("Loading weather data...");
            }

            // 今日は何の日セクション
            ui.add_space(20.0);
            ui.heading("What day is it today?");
            ui.collapsing("Anniversary in Japan", |ui| {
                if let Some(anniversaries) = &self.jp_anniversary {
                    for anniv in anniversaries {
                        ui.label(format!("・{}: {}", anniv.name, anniv.description));
                    }
                } else {
                    ui.label("Loading anniversary data...");
                }
            });

            ui.collapsing("Anniversary in the world", |ui| {
                if let Some(holidays) = &self.world_holiday {
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

            // 今日の格言セクション
            ui.add_space(20.0);
            ui.heading("Quote of the day");
            if let Some(joke) = &self.joke {
                ui.label(joke);
            } else {
                ui.label("Loading quote data...");
            }

            // ブックマークセクション
            ui.add_space(20.0);
            egui::Grid::new("bookmarks_grid")
                .num_columns(2)
                .show(ui, |ui| {
                    // デスクトップアプリのブックマーク
                    ui.vertical(|ui| {
                        ui.heading("Desktop Applications");
                        for category in &self.config.bookmarks.desktop {
                            ui.collapsing(&category.name, |ui| {
                                for bookmark in &category.items {
                                    if ui.button(&bookmark.name).clicked() {
                                        // ここでデスクトップアプリを起動するコードを追加
                                        #[cfg(target_os = "windows")]
                                        {
                                            let _ = std::process::Command::new("cmd")
                                                .args(&["/C", &bookmark.url])
                                                .spawn();
                                        }
                                        #[cfg(target_os = "linux")]
                                        {
                                            let _ = std::process::Command::new("sh")
                                                .arg("-c")
                                                .arg(&bookmark.url)
                                                .spawn();
                                        }
                                        #[cfg(target_os = "macos")]
                                        {
                                            let _ = std::process::Command::new("open")
                                                .arg(&bookmark.url)
                                                .spawn();
                                        }
                                    }
                                }
                            });
                        }
                    });

                    ui.end_row();

                    // WEBアプリのブックマーク
                    ui.vertical(|ui| {
                        ui.heading("WEB Applications");
                        for category in &self.config.bookmarks.web {
                            ui.collapsing(&category.name, |ui| {
                                for bookmark in &category.items {
                                    if ui.button(&bookmark.name).clicked() {
                                        // ここでWEBページを開くコードを追加
                                        #[cfg(target_os = "windows")]
                                        {
                                            let _ = std::process::Command::new("cmd")
                                                .args(&["/C", "start", &bookmark.url])
                                                .spawn();
                                        }
                                        #[cfg(target_os = "linux")]
                                        {
                                            let _ = std::process::Command::new("xdg-open")
                                                .arg(&bookmark.url)
                                                .spawn();
                                        }
                                        #[cfg(target_os = "macos")]
                                        {
                                            let _ = std::process::Command::new("open")
                                                .arg(&bookmark.url)
                                                .spawn();
                                        }
                                    }
                                }
                            });
                        }
                    });
                });

            // TODOセクション
            ui.add_space(20.0);
            ui.heading("Todays TODO");
            if let Some(todos) = &self.todos {
                for (id, title) in todos {
                    ui.horizontal(|ui| {
                        if ui.button("✅").clicked() {
                            // ここでタスクを完了させるコードを追加
                            self.mark_todo_completed(id);
                        }
                        ui.label(title);
                    });
                }
            } else if self.config.notion_api_key.is_some() {
                ui.label("Loading TODOs...");
            } else {
                ui.label("Notion APIキーが設定されていません");
                if ui.button("設定を開く").clicked() {
                    // 設定画面を開く処理をここに追加
                }
            }

            // 設定ボタン
            ui.add_space(20.0);
            if ui.button("Settings").clicked() {
                // 設定画面を開く処理をここに追加
            }
        });
    }
}

impl MyApp {
    // データ更新用メソッド
    fn update_data(&mut self) {
        let config = self.config.clone();
        let client = self.client.clone();

        // 天気データの取得
        if self.weather_data.is_none() {
            let client = client.clone();
            let api_key = config.weather_api_key.clone();
            let location = config.location.clone();

            self.runtime.spawn(async move {
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

        // 格言データの取得
        if self.joke.is_none() {
            let client = client.clone();

            self.runtime.spawn(async move {
                match client
                    .get("https://official-joke-api.appspot.com/jokes/random")
                    .send()
                    .await
                {
                    Ok(response) => match response.json::<JokeResponse>().await {
                        Ok(joke) => Some(format!("{} - {}", joke.setup, joke.punchline)),
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            });
        }

        // 日本の記念日データの取得
        if self.jp_anniversary.is_none() {
            let client = client.clone();
            let now: DateTime<Local> = Local::now();
            let month = now.month();
            let day = now.day();

            self.runtime.spawn(async move {
                let url = format!(
                    "https://api.whatistoday.cyou/v3/anniv/month/{}/day/{}",
                    month, day
                );

                match client.get(&url).send().await {
                    Ok(response) => match response.json::<JapaneseAnniversaryResponse>().await {
                        Ok(data) => Some(data.anniversaries),
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            });
        }

        // 世界の祝日データの取得
        if self.world_holiday.is_none() {
            let client = client.clone();
            let now: DateTime<Local> = Local::now();
            let year = now.year();
            let country_code = config.country_code.clone();

            self.runtime.spawn(async move {
                let url = format!(
                    "https://date.nager.at/api/v3/publicholidays/{}/{}",
                    year, country_code
                );

                match client.get(&url).send().await {
                    Ok(response) => match response.json::<Vec<HolidayResponse>>().await {
                        Ok(data) => Some(data),
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            });
        }

        // Notionのタスクデータの取得
        if self.todos.is_none()
            && self.config.notion_api_key.is_some()
            && self.config.notion_database_id.is_some()
        {
            let client = client.clone();
            let api_key = self.config.notion_api_key.clone().unwrap();
            let database_id = self.config.notion_database_id.clone().unwrap();

            self.runtime.spawn(async move {
                let url = format!("https://api.notion.com/v1/databases/{}/query", database_id);

                // NotionのAPIを呼び出す
                let response = client
                    .post(&url)
                    .header("Authorization", format!("Bearer {}", api_key))
                    .header("Notion-Version", "2021-08-16")
                    .json(&serde_json::json!({
                        "filter": {
                            "property": "status",
                            "select": {
                                "equals": "To Do"
                            }
                        }
                    }))
                    .send()
                    .await;

                match response {
                    Ok(res) => match res.json::<NotionTodoResponse>().await {
                        Ok(data) => {
                            // タスクの情報を抽出
                            let mut todos = Vec::new();
                            for page in data.results {
                                if !page.properties.title.title.is_empty() {
                                    let title = page.properties.title.title[0].plain_text.clone();
                                    todos.push((page.id, title));
                                }
                            }
                            Some(todos)
                        }
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            });
        }
    }

    // タスクを完了としてマークするメソッド
    fn mark_todo_completed(&self, task_id: &str) {
        if let (Some(api_key), Some(_)) =
            (&self.config.notion_api_key, &self.config.notion_database_id)
        {
            let client = self.client.clone();
            let api_key = api_key.clone();
            let task_id = task_id.to_string();

            self.runtime.spawn(async move {
                let url = format!("https://api.notion.com/v1/pages/{}", task_id);

                // タスクのステータスを更新
                let _ = client
                    .patch(&url)
                    .header("Authorization", format!("Bearer {}", api_key))
                    .header("Notion-Version", "2021-08-16")
                    .json(&serde_json::json!({
                        "properties": {
                            "status": {
                                "select": {
                                    "name": "Done"
                                }
                            }
                        }
                    }))
                    .send()
                    .await;
            });
        }
    }
}

fn main() {
    let app = MyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
