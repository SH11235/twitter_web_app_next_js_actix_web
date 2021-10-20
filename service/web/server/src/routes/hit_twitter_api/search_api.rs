use crate::driver::hit_twitter_api::TwitterApiDriver;
use crate::usecase::hit_twitter_api::hit_search_api::{self, RequestParams};
use actix_web::{web, HttpRequest, HttpResponse};
use qstring::QString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    pub q: String,
    pub count: Option<String>,
    pub result_type: Option<String>,
    pub origin: Option<String>,
    pub max_id: Option<String>,
}

impl GetParams {
    pub fn to_api_params(&self) -> RequestParams {
        RequestParams {
            q: self.q.clone(),
            count: match self.count.clone() {
                Some(count) => count,
                None => "100".to_string(),
            },
            result_type: match self.result_type.clone() {
                Some(result_type) => result_type,
                None => "mixed".to_string(),
            },
            origin: match self.origin.clone() {
                Some(o) => o,
                None => "".to_string(),
            },
            max_id: match self.max_id.clone() {
                None => "0".to_string(),
                Some(params) => {
                    let qs = QString::from(params.as_str());
                    qs.get("max_id").unwrap_or("0").to_string()
                }
            },
        }
    }
}

pub async fn route(req: HttpRequest, item: web::Query<GetParams>) -> HttpResponse {
    // TODO CORS対応
    let _origin = match req.headers().get("Origin") {
        Some(o) => o.to_str().unwrap().to_string(),
        None => "".to_string(),
    };

    let twitter_api_driver = TwitterApiDriver::new();
    match hit_search_api::execute(twitter_api_driver, item.to_api_params()).await {
        Ok(res) => HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .json(res),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
