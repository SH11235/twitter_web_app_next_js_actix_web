use serde::Deserialize;
#[derive(Queryable)]
pub struct Tweet {
    pub id: i32,
    pub text: String,
    pub tweetlink: String,
    pub userlink: String,
    pub tweettime: String,
    pub username: String,
    pub screenname: String,
    pub profileimageurl: String,
}

use super::schema::tweets;

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "tweets"]
pub struct NewTweet {
    pub text: String,
    pub tweetlink: String,
    pub userlink: String,
    pub tweettime: String,
    pub username: String,
    pub screenname: String,
    pub profileimageurl: String,
}
