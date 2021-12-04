use crate::driver::hit_twitter_api::TwitterApiDriver;
use crate::usecase::hit_twitter_api::hit_search_api::{self, RequestParams};
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResultType {
    Popular,
    Mixed,
    Recent,
}

impl fmt::Display for ResultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResultType::Popular => write!(f, "popular"),
            ResultType::Mixed => write!(f, "mixed"),
            ResultType::Recent => write!(f, "recent"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    pub q: String,
    pub count: Option<i32>,
    pub result_type: Option<ResultType>,
    pub max_id: Option<i32>,
}

impl GetParams {
    pub fn to_api_params(&self) -> RequestParams {
        RequestParams {
            q: self.q.clone(),
            count: match self.count {
                Some(count) => count.to_string(),
                None => 100.to_string(),
            },
            result_type: match &self.result_type {
                Some(result_type) => result_type.to_string(),
                None => "mixed".to_string(),
            },
            max_id: match self.max_id {
                None => 0.to_string(),
                Some(max_id) => max_id.to_string(),
            },
        }
    }
}

pub async fn route(item: web::Query<GetParams>) -> HttpResponse {
    let twitter_api_driver = TwitterApiDriver::new();
    match hit_search_api::execute(twitter_api_driver, item.to_api_params()).await {
        Ok(res) => HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .json(res),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
