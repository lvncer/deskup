// src/services/anniversary.rs
// 記念日サービス

use chrono::{Datelike, Local};
use reqwest::Client;
use tokio::runtime::Runtime;

use crate::models::anniversary::{JapaneseAnniversary, JapaneseAnniversaryResponse};

// 日本の記念日データの取得
pub fn fetch_anniversary(
    runtime: &mut Runtime,
    client: &Client,
    jp_anniversary: &mut Option<Vec<JapaneseAnniversary>>,
) {
    if jp_anniversary.is_none() {
        let client = client.clone();
        let now = Local::now();
        let month = now.month();
        let day = now.day();

        runtime.spawn(async move {
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
}
