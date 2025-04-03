// src/services/holiday.rs
// 祝日サービス

use chrono::{Datelike, Local};
use reqwest::Client;
use tokio::runtime::Runtime;

use crate::models::holiday::HolidayResponse;

// 世界の祝日データの取得
pub fn fetch_holiday(
    runtime: &mut Runtime,
    client: &Client,
    country_code: &str,
    world_holiday: &mut Option<Vec<HolidayResponse>>,
) {
    if world_holiday.is_none() {
        let client = client.clone();
        let now = Local::now();
        let year = now.year();
        let country_code = country_code.to_string();

        runtime.spawn(async move {
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
}
