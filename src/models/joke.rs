// src/models/joke.rs
// 格言のモデル

use serde::Deserialize;

// 格言API用の構造体
#[derive(Deserialize, Debug)]
pub struct JokeResponse {
    pub setup: String,
    pub punchline: String,
}
