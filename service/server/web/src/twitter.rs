use actix_web::{HttpRequest, HttpResponse};
use dotenv::dotenv;
use qstring::QString;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::Value::Array;
use std::env;
#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    search_metadata: Value,
    statuses: Vec<Value>,
}

struct Twitter {
    q: String,
    count: String,
    result_type: String,
    origin: String,
    bearer_token: String,
}

impl Twitter {
    pub fn new(_req: HttpRequest) -> Self {
        // .envファイルのトークンの値を読み込む
        dotenv().ok();
        let query_str = _req.query_string();
        let qs = QString::from(query_str);
        Twitter {
            q: qs.get("q").unwrap().to_string(),
            count: "100".to_string(),
            result_type: qs.get("type").unwrap().to_string(),
            origin: match _req.headers().get("Origin") {Some(o) => o.to_str().unwrap().to_string(), None => "".to_string()},
            bearer_token: env::var("bearer_token").expect("bearer_token is not found")
        }
    }

    pub async fn search(
        &self,
        next_results: &String
    ) -> Result<SearchResult, Box<dyn std::error::Error>> {
        let endpoint = "https://api.twitter.com/1.1/search/tweets.json";
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", &self.bearer_token).parse().unwrap(),
        );
        let client = reqwest::Client::new()
            .get(endpoint)
            .query(&next_results)
            .headers(headers);
        let res: SearchResult = client.send().await?.json().await?;
        Ok(res)
    }
}

pub async fn run_search(req: HttpRequest) -> HttpResponse {
    let twitter = Twitter::new(req);

    // CORS対応
    let allowed_origin_list = [
        "http://localhost:3000",
        "http://localhost:8000",
        "http://ec2-3-135-220-104.us-east-2.compute.amazonaws.com:3000",
    ];
    let mut allow_origin = false;
    for origin in allowed_origin_list.iter() {
        if origin == &twitter.origin {
            allow_origin = true;
            break;
        }
    }

    let mut values: Vec<Value> = vec![];
    let mut result : Result<SearchResult, Box<dyn std::error::Error>>;
    let mut next_results = format!("?q={}&count={}&result_type={}",&twitter.q,&twitter.count,&twitter.result_type);

    for _ in 0..10 {
        result = twitter.search(&next_results).await;
        match result {
            Ok(res) => {
                next_results = res.search_metadata["next_results"].as_str().unwrap().to_string();
                values.push(Array(res.statuses));
            },
            Err(err) => println!("{:?}", err),
        }
    }

    if allow_origin {
        HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Methods", "GET")
            .header("Access-Control-Allow-Origin", "*")
            .json(values)
    } else {
        HttpResponse::InternalServerError().body(format!("Access from origin {} has been blocked by CORS policy: No 'Access-Control-Allow-Origin' header is present on the requested resource.", &twitter.origin))
    }
}
