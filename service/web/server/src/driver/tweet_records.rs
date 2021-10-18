use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::tweet_record::TweetRecord;
use crate::schema::tweets;
use crate::usecase::tweet_records::{add, search_by_tweet_id};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub struct TweetRecordDriver<'a> {
    connection: &'a PgConnection,
}

impl<'a> UseCase for TweetRecordDriver<'a> {}

impl<'a> TweetRecordDriver<'a> {
    pub fn new(connection: &'a PgConnection) -> TweetRecordDriver<'a> {
        TweetRecordDriver { connection }
    }
}

#[derive(Insertable)]
#[table_name = "tweets"]
struct NewRecord {
    tweet_id: String,
    text: String,
    tweet_link: String,
    user_link: String,
    tweet_time: String,
    user_name: String,
    screen_name: String,
    profile_image_url: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
struct RecordItem {
    id: i32,
    tweet_id: String,
    text: String,
    tweet_link: String,
    user_link: String,
    tweet_time: String,
    user_name: String,
    screen_name: String,
    profile_image_url: String,
}

impl RecordItem {
    fn to_entity(&self) -> TweetRecord {
        TweetRecord {
            id: self.id,
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

impl<'a> add::AddRecordUseCase for TweetRecordDriver<'a> {
    fn add_record(&self, input: add::InputData) -> Result<TweetRecord, DataAccessError> {
        let new_record = NewRecord {
            tweet_id: input.tweet_id,
            text: input.text,
            tweet_link: input.tweet_link,
            user_link: input.user_link,
            tweet_time: input.tweet_time,
            user_name: input.user_name,
            screen_name: input.screen_name,
            profile_image_url: input.profile_image_url,
        };

        let record_result = diesel::insert_into(tweets::table)
            .values(new_record)
            .get_result::<RecordItem>(self.connection)
            .map_err(|_| DataAccessError::InternalError)?;

        Ok(record_result.to_entity())
    }
}

impl<'a> search_by_tweet_id::SearchRecordsByTweetIdUseCase for TweetRecordDriver<'a> {
    fn get_records(
        &self,
        input: search_by_tweet_id::InputData,
    ) -> Result<Vec<TweetRecord>, DataAccessError> {
        let tweet_id = input.tweet_id;

        let record_results: Vec<RecordItem> = tweets::dsl::tweets
            .filter(tweets::dsl::tweet_id.eq(tweet_id))
            .load::<RecordItem>(self.connection)
            .map_err(|_| DataAccessError::InternalError)?;

        let results = record_results
            .iter()
            .map(|result| result.to_entity())
            .collect();

        Ok(results)
    }
}
