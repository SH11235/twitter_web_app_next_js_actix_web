// external crate
use actix_web::{post, web, HttpRequest, HttpResponse};
use dotenv::dotenv;
use qstring::QString;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

// lib.rs
use super::{establish_connection, register_api_result, register_tweet_to_db, NewTweet};

#[derive(Debug, Serialize, Deserialize)]
struct SearchAPIResult {
    search_metadata: Value,
    statuses: Vec<TweetInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TweetInfo {
    pub text: String,
    pub user: TweetUser,
    pub id_str: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetUser {
    pub name: String,
    pub screen_name: String,
    pub profile_image_url_https: String,
}

struct Twitter {
    q: String,
    count: String,
    result_type: String,
    origin: String,
    bearer_token: String,
}

struct ApiParams {
    q: String,
    result_type: String,
    origin: String,
}

impl Twitter {
    pub fn new(params: ApiParams) -> Self {
        // .envファイルのトークンの値を読み込む
        dotenv().ok();
        Twitter {
            q: params.q,
            count: "100".to_string(),
            result_type: params.result_type,
            origin: params.origin,
            bearer_token: env::var("bearer_token").expect("bearer_token is not found"),
        }
    }

    pub async fn hit_search_api(
        &self,
        next_results: Option<&String>,
    ) -> Result<SearchAPIResult, Box<dyn std::error::Error>> {
        let endpoint = "https://api.twitter.com/1.1/search/tweets.json";
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", &self.bearer_token).parse().unwrap(),
        );
        let max_id = match next_results {
            None => "".to_string(),
            Some(params) => {
                let qs = QString::from(params.as_str());
                qs.get("max_id").unwrap_or("0").to_string()
            }
        };
        let client = reqwest::Client::new()
            .get(endpoint)
            .query(&[
                ("q", &self.q),
                ("count", &self.count),
                ("result_type", &self.result_type),
                ("max_id", &max_id),
            ])
            .headers(headers);
        let res: SearchAPIResult = client.send().await?.json().await?;
        Ok(res)
    }
}

pub async fn run_search(req: HttpRequest) -> HttpResponse {
    let qs = QString::from(req.query_string());
    let params = ApiParams {
        q: qs.get("q").unwrap().to_string(),
        result_type: qs.get("type").unwrap().to_string(),
        origin: match req.headers().get("Origin") {
            Some(o) => o.to_str().unwrap().to_string(),
            None => "".to_string(),
        },
    };
    let twitter = Twitter::new(params);

    // CORS対応
    let allowed_origin_list = [
        "http://localhost:3000",
        "http://localhost:8000",
        "http://ec2-3-135-220-104.us-east-2.compute.amazonaws.com:3000",
    ];
    let mut allow_origin = false;
    let req_origin = &twitter.origin;
    for origin in allowed_origin_list.iter() {
        if origin == req_origin {
            allow_origin = true;
            break;
        }
    }
    if req_origin.is_empty() {
        allow_origin = true;
    }

    if !allow_origin {
        return HttpResponse::InternalServerError().body(format!("Access from origin {} has been blocked by CORS policy: No 'Access-Control-Allow-Origin' header is present on the requested resource.", &twitter.origin));
    }

    let mut values: Vec<TweetInfo> = vec![];
    let mut result: Result<SearchAPIResult, Box<dyn std::error::Error>>;
    let mut next_results = format!(
        "?q={}&count={}&result_type={}",
        &twitter.q, &twitter.count, &twitter.result_type
    );

    for _ in 0..10 {
        result = twitter.hit_search_api(Some(&next_results)).await;
        match result {
            Ok(res) => {
                next_results = res.search_metadata["next_results"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                values.extend(res.statuses);
            }
            Err(err) => {
                return HttpResponse::InternalServerError().body(err.to_string());
            }
        }
    }

    HttpResponse::Ok()
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Methods", "GET")
        .header("Access-Control-Allow-Origin", "*")
        .json(values)
}

pub async fn hit_api_and_register_tweet(req: HttpRequest) -> HttpResponse {
    let qs = QString::from(req.query_string());
    let params = ApiParams {
        q: qs.get("q").unwrap().to_string(),
        result_type: qs.get("type").unwrap().to_string(),
        origin: match req.headers().get("Origin") {
            Some(o) => o.to_str().unwrap().to_string(),
            None => "".to_string(),
        },
    };
    let result = Twitter::new(params)
        .hit_search_api(None)
        .await
        .unwrap();
    let tweets = result.statuses;
    let connection = establish_connection();
    let _register_api_result = register_api_result(&connection, &tweets);

    HttpResponse::Ok().json(&tweets)
}

#[post("/register_favorite_tweet")]
pub async fn register_favorite_tweet(tweet: web::Json<NewTweet>) -> String {
    let connection = establish_connection();
    let favorite_tweet = NewTweet {
        text: tweet.text.to_string(),
        tweet_link: tweet.tweet_link.to_string(),
        user_link: tweet.user_link.to_string(),
        tweet_time: tweet.tweet_time.to_string(),
        user_name: tweet.user_name.to_string(),
        screen_name: tweet.screen_name.to_string(),
        profile_image_url: tweet.profile_image_url.to_string(),
    };
    let _register_api_result = register_tweet_to_db(&connection, favorite_tweet);
    "register tweet".to_string()
}
