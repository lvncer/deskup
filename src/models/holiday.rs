// src/models/holiday.rs
// 祝日のモデル

use serde::Deserialize;

// 世界の祝日API用の構造体
#[derive(Deserialize, Debug)]
pub struct HolidayResponse {
    pub date: String,
    pub name: String,
    #[serde(rename = "localName")]
    pub local_name: String,
}
