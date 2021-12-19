use crate::database_utils::error::DataAccessError;
use crate::domain::entity::tweet_record::TweetRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub tweet_id: String,
}
pub trait SearchRecordsByTweetIdUseCase {
    fn get_records(&self, input: InputData) -> Result<Vec<TweetRecord>, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<Vec<TweetRecord>, DataAccessError>
where
    T: SearchRecordsByTweetIdUseCase,
{
    data_access.get_records(input)
}
