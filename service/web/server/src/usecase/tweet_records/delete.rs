use crate::database_utils::error::DataAccessError;

#[derive(Debug)]
pub struct InputData {
    pub tweet_id: String,
}
// pub trait SearchRecordsByTweetIdUseCase {
//     fn get_records(&self, input: InputData) -> Result<Vec<TweetRecord>, DataAccessError>;
// }
pub trait DeleteRecordUseCase {
    fn delete_record(&self, input: InputData) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: DeleteRecordUseCase,
{
    data_access.delete_record(input)
}
