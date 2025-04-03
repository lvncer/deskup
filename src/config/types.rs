// src/config/types.rs
// 設定関連の型定義

use serde::{Deserialize, Serialize};

// ユーザー設定を格納する構造体
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub user_name: String,
    pub weather_api_key: String,
    pub location: String,
    pub country_code: String,
    pub bookmarks: Bookmarks,
    pub notion_api_key: Option<String>,
    pub notion_database_id: Option<String>,
}

impl AppConfig {
    // デフォルト設定を生成（ユーザー名指定）
    pub fn default_with_username(username: String) -> Self {
        Self {
            user_name: username,
            weather_api_key: "your_api_key_here".to_string(),
            location: "Tokyo".to_string(),
            country_code: "JP".to_string(),
            bookmarks: Bookmarks {
                desktop: vec![BookmarkCategory {
                    name: "作業".to_string(),
                    items: vec![Bookmark {
                        name: "notepad".to_string(),
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
        }
    }
}

// ブックマークを格納する構造体
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bookmarks {
    pub desktop: Vec<BookmarkCategory>,
    pub web: Vec<BookmarkCategory>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BookmarkCategory {
    pub name: String,
    pub items: Vec<Bookmark>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bookmark {
    pub name: String,
    pub url: String,
}
