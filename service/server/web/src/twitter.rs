use super::{establish_connection, write_tweet_to_db};
// use crate::models::NewTweet;
use actix_web::{HttpRequest, HttpResponse};
use dotenv::dotenv;
use qstring::QString;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    search_metadata: Value,
    statuses: Vec<Tweet>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub text: String,
    pub user: User,
    pub id_str: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub screen_name: String,
    pub profile_image_url_https: String,
}

struct Twitter {}

impl Twitter {
    pub fn new() -> Self {
        Twitter {}
    }

    pub async fn search(
        &self,
        _req: &HttpRequest,
    ) -> Result<SearchResult, Box<dyn std::error::Error>> {
        let endpoint = "https://api.twitter.com/1.1/search/tweets.json";
        let mut headers = HeaderMap::new();
        // .envファイルのトークンの値を読み込む
        dotenv().ok();
        let bearer_token = env::var("bearer_token").expect("bearer_token is not found");
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", bearer_token).parse().unwrap(),
        );
        let query_str = _req.query_string();
        let qs = QString::from(query_str);
        let q = qs.get("q").unwrap();
        let count = "100";
        let result_type = qs.get("type").unwrap();

        let client = reqwest::Client::new()
            .get(endpoint)
            .query(&[("q", q), ("count", count), ("result_type", result_type)])
            .headers(headers);
        let res: SearchResult = client.send().await?.json().await?;
        Ok(res)
    }
}

pub async fn run_search(req: HttpRequest) -> HttpResponse {
    let result = Twitter::new().search(&req).await;
    // CORS対応
    let allowed_origin_list = [
        "http://localhost:3000",
        "http://localhost:8000",
        "http://ec2-3-135-220-104.us-east-2.compute.amazonaws.com:3000",
    ];
    let mut allow_origin = false;
    let req_origin = match &req.headers().get("Origin") {
        Some(o) => o.to_str().unwrap(),
        None => {
            allow_origin = true; // localhost:8000に直接アクセスするとOriginがNullになるのでこの場合は許可する
            ""
        }
    };
    for origin in allowed_origin_list.iter() {
        if origin == &req_origin {
            allow_origin = true;
            break;
        }
    }

    match result {
        Ok(json) => {
            if allow_origin {
                HttpResponse::Ok()
                    .header("Content-Type", "application/json")
                    .header("Access-Control-Allow-Methods", "GET")
                    .header("Access-Control-Allow-Origin", "*")
                    .json(json)
            } else {
                HttpResponse::InternalServerError().body(format!("Access from origin {} has been blocked by CORS policy: No 'Access-Control-Allow-Origin' header is present on the requested resource.", req_origin))
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn write_tweet(req: HttpRequest) -> HttpResponse {
    let result = Twitter::new().search(&req).await.unwrap();
    let tweets = result.statuses;
    // let mut tweet_vec: Vec<NewTweet>;
    // let twitter_base_URL = "https://twitter.com";
    // for tweet in tweets.iter() {
    //     let text = &tweet.text;
    //     let user_link = format!("{}{}{}", &twitter_base_URL, "/", tweet.user.screen_name);
    //     let tweet_link = format!("{}{}{}", user_link, "/status/", tweet.id_str);
    //     let tweet_time = &tweet.created_at;
    //     let user_name = &tweet.user.name;
    //     let screen_name = &tweet.user.screen_name;
    //     let profile_image_url = &tweet.user.profile_image_url_https;

    //     tweet_vec.push(NewTweet {
    //         text: &text.clone(),
    //         tweetlink: &tweet_link,
    //         userlink: &user_link,
    //         tweettime: &tweet_time,
    //         username: &user_name,
    //         screenname: &screen_name,
    //         profileimageurl: &profile_image_url,
    //     });
    // }
    println!("{:?}", tweets.len());

    let connection = establish_connection();
    let _write_tweet_to_db = write_tweet_to_db(&connection, &tweets);
    // println!("{:?}", write);

    HttpResponse::Ok().json(&tweets)
}
