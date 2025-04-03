// src/services/joke.rs
// 格言サービス

use reqwest::Client;
use tokio::runtime::Runtime;

use crate::models::joke::JokeResponse;

// 格言データの取得
pub fn fetch_joke(runtime: &mut Runtime, client: &Client, joke: &mut Option<String>) {
    if joke.is_none() {
        let client = client.clone();

        runtime.spawn(async move {
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
}
