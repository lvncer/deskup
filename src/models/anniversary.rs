// src/models/anniversary.rs
// 記念日のモデル

use serde::Deserialize;

// 日本の記念日API用の構造体
#[derive(Deserialize, Debug)]
pub struct JapaneseAnniversaryResponse {
    pub anniversaries: Vec<JapaneseAnniversary>,
}

#[derive(Deserialize, Debug)]
pub struct JapaneseAnniversary {
    pub name: String,
    pub description: String,
}
