use crate::database_utils::pool::DbPool;
use crate::driver::tweet_records::TweetRecordDriver;
use crate::usecase::tweet_records::add::{self, InputData};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostParams {
    pub text: String,
    pub tweet_link: String,
    pub user_link: String,
    pub tweet_time: String,
    pub user_name: String,
    pub screen_name: String,
    pub profile_image_url: String,
}

impl PostParams {
    pub fn to_input_data(&self) -> InputData {
        InputData {
            text: self.text.clone(),
            tweet_link: self.tweet_link.clone(),
            user_link: self.user_link.clone(),
            tweet_time: self.tweet_time.clone(),
            user_name: self.user_name.clone(),
            screen_name: self.screen_name.clone(),
            profile_image_url: self.profile_image_url.clone(),
        }
    }
}

pub async fn route(pool: web::Data<DbPool>, item: web::Json<PostParams>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let tweet_driver = TweetRecordDriver::new(&connection);

    match add::execute(tweet_driver, item.to_input_data()) {
        Ok(category) => HttpResponse::Created().json(category),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
