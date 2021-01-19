use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use dotenv::dotenv;
use env_logger;
use qstring::QString;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    search_metadata: Value,
    statuses: Vec<Value>,
}
#[derive(Deserialize, Serialize)]
struct QueryObject {
    q: String,
    count: u32,
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

async fn twitter_search(req: HttpRequest) -> HttpResponse {
    let result = Twitter::new().search(&req).await;
    // CORS対応
    let allowed_origin_list = [
        "http://localhost:3000",
        "http://ec2-18-191-199-143.us-east-2.compute.amazonaws.com:3000",
    ];
    let req_origin = match &req.headers().get("Origin") {
        Some(o) => o.to_str().unwrap(),
        None => "",
    };
    let mut allow_origin = false;
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
                    .header("Access-Control-Allow-Origin", req_origin)
                    .json(json)
            } else {
                HttpResponse::InternalServerError().body(format!("Access from origin {} has been blocked by CORS policy: No 'Access-Control-Allow-Origin' header is present on the requested resource.", req_origin))
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/twitter_search").route(web::get().to(twitter_search)))
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run()
    .await
}
