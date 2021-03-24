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

#[derive(Insertable, Debug)]
#[table_name = "tweets"]
pub struct NewTweet<'a> {
    pub text: &'a str,
    pub tweetlink: &'a str,
    pub userlink: &'a str,
    pub tweettime: &'a str,
    pub username: &'a str,
    pub screenname: &'a str,
    pub profileimageurl: &'a str,
}
