use crate::database_utils::error::DataAccessError;
use crate::domain::entity::tweet_record::TweetRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub tweet_id: String,
    pub text: String,
    pub tweet_link: String,
    pub user_link: String,
    pub tweet_time: String,
    pub user_name: String,
    pub screen_name: String,
    pub profile_image_url: String,
}

pub trait AddRecordUseCase {
    fn add_record(&self, input: InputData) -> Result<TweetRecord, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<TweetRecord, DataAccessError>
where
    T: AddRecordUseCase,
{
    data_access.add_record(input)
}
