// external crate
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// modules
pub mod models;
pub mod schema;
pub mod twitter;
use models::{NewTweet, Tweet};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn register_api_result(conn: &PgConnection, tweets: &[twitter::TweetInfo]) {
    let twitter_base_url = "https://twitter.com";
    for tweet in tweets.iter() {
        let text = &tweet.text;
        let user_link = format!("{}{}{}", &twitter_base_url, "/", tweet.user.screen_name);
        let tweet_link = format!("{}{}{}", user_link, "/status/", tweet.id_str);
        let tweet_time = &tweet.created_at;
        let user_name = &tweet.user.name;
        let screen_name = &tweet.user.screen_name;
        let profile_image_url = &tweet.user.profile_image_url_https;

        let tweet_vec = NewTweet {
            text: text.to_string(),
            tweet_link,
            user_link,
            tweet_time: tweet_time.to_string(),
            user_name: user_name.to_string(),
            screen_name: screen_name.to_string(),
            profile_image_url: profile_image_url.to_string(),
        };
        register_tweet_to_db(conn, tweet_vec);
    }
}

pub fn register_tweet_to_db(conn: &PgConnection, tweet: NewTweet) {
    use schema::tweets;
    let _insert_tweet: Tweet = diesel::insert_into(tweets::table)
        .values(&tweet)
        .get_result(conn)
        .expect("Error saving new post");
}
