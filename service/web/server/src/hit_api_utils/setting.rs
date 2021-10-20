use dotenv::dotenv;
use std::env;

pub fn search_api_url() -> String {
    "https://api.twitter.com/1.1/search/tweets.json".to_string()
}

pub fn bearer_token() -> String {
    dotenv().ok();
    env::var("bearer_token").expect("bearer_token is not found")
}

pub fn twitter_base_url() -> String {
    "https://twitter.com".to_string()
}
