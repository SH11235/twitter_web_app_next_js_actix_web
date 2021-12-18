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

pub const MAX_TWEET_NUM: i32 = 100;

pub fn get_request_num(count: i32) -> i32 {
    let request_num: i32 = if (count % MAX_TWEET_NUM) == 0 {
        count / MAX_TWEET_NUM
    } else {
        count / MAX_TWEET_NUM + 1
    };
    request_num
}

#[test]
fn check_req_num() {
    assert_eq!(get_request_num(200), 2);
    assert_eq!(get_request_num(201), 3);
}
