use dotenv::dotenv;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::Deserialize;
use serde_json::Value;
use std::env;

#[derive(Deserialize, Debug)]
struct SearchResult {
    search_metadata: Value,
    statuses: Vec<Value>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "https://api.twitter.com/1.1/search/tweets.json?lang=ja";
    let mut headers = HeaderMap::new();
    // .envファイルの値を読み込む
    dotenv().ok();
    let bearer_token = env::var("bearer_token").expect("bearer_token is not found");
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", bearer_token).parse().unwrap(),
    );
    let client = reqwest::Client::new()
        .get(endpoint)
        .query(&[("q", "かき氷"), ("count", "5")])
        .headers(headers);
    let res: SearchResult = client.send().await?.json().await?;
    println!("{:#?}", res);
    Ok(())
}
