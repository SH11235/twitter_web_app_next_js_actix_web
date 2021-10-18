use crate::database_utils::pool::DbPool;
use crate::driver::tweet_records::TweetRecordDriver;
use crate::usecase::tweet_records::search_by_tweet_id::{self, InputData};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    pub tweet_id: String,
}

impl GetParams {
    pub fn to_input_data(&self) -> InputData {
        InputData {
            tweet_id: self.tweet_id.clone(),
        }
    }
}

pub async fn route(pool: web::Data<DbPool>, item: web::Query<GetParams>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let tweet_driver = TweetRecordDriver::new(&connection);

    match search_by_tweet_id::execute(tweet_driver, item.to_input_data()) {
        Ok(category) => HttpResponse::Created().json(category),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
