use crate::database_utils::pool::DbPool;
use crate::driver::tweet_records::TweetRecordDriver;
use crate::usecase::tweet_records::delete::{self, InputData};

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteParams {
    pub tweet_id: String,
}

impl DeleteParams {
    pub fn to_input_data(&self) -> InputData {
        InputData {
            tweet_id: self.tweet_id.clone(),
        }
    }
}

pub async fn route(pool: web::Data<DbPool>, item: web::Query<DeleteParams>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let attendance_driver = TweetRecordDriver::new(&connection);

    match delete::execute(attendance_driver, item.to_input_data()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
