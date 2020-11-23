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
    eprintln!("*** 開始 ***");
    let endpoint = "https://api.twitter.com/1.1/search/tweets.json?lang=ja";
    let mut headers = HeaderMap::new();
    dotenv().ok(); // ここで.envファイルの値を読み込んでいます。
    let bearer_token = env::var("bearer_token").expect("bearer_token is not found");
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", bearer_token).parse().unwrap(),
    );
    let client = reqwest::Client::new()
        .get(endpoint)
        .query(&[("q", "かき氷"), ("count", "5")])
        .headers(headers);
    let res = client.send().await?;
    // println!("{:#?}", res);
    let body: SearchResult = res.json().await?;

    println!("{:#?}", body);

    eprintln!("*** 終了 ***");
    Ok(())
}
