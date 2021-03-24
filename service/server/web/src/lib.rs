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
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn register_tweet_to_db<'a>(conn: &PgConnection, tweets: &Vec<twitter::TweetInfo>) {
    use schema::tweets;

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
            text: &text,
            tweetlink: &tweet_link,
            userlink: &user_link,
            tweettime: &tweet_time,
            username: &user_name,
            screenname: &screen_name,
            profileimageurl: &profile_image_url,
        };
        println!("{:?}", tweet_vec);
        let _insert_tweet: Tweet = diesel::insert_into(tweets::table)
            .values(&tweet_vec)
            .get_result(conn)
            .expect("Error saving new post");
    }
}
