use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetRecord {
    pub id: i32,
    pub tweet_id: String,
    pub text: String,
    pub tweet_link: String,
    pub user_link: String,
    pub tweet_time: String,
    pub user_name: String,
    pub screen_name: String,
    pub profile_image_url: String,
}
