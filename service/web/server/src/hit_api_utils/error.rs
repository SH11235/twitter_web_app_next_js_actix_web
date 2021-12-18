use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum TwitterAPIAccessError {
    InternalError,
    InternalErrorWithMessage(String),
}

impl StdError for TwitterAPIAccessError {}

impl fmt::Display for TwitterAPIAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TwitterAPIAccessError::InternalError => {
                write!(f, "Twitter API access Error in use case!")
            }
            TwitterAPIAccessError::InternalErrorWithMessage(message) => write!(f, "{}", message),
        }
    }
}

pub trait UseCase {
    fn parse_data_access_result<T>(
        &self,
        result: Result<T, Box<dyn StdError>>,
    ) -> Result<T, TwitterAPIAccessError> {
        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(TwitterAPIAccessError::InternalError),
        }
    }
}
