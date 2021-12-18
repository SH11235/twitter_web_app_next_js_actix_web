use crate::domain::entity::search_api::Tweet;
use crate::hit_api_utils::error::TwitterAPIAccessError;
use async_trait::async_trait;

#[derive(Debug)]
pub struct RequestParams {
    pub q: String,
    pub count: String,
    pub result_type: String,
    pub lang: String,
    pub max_id: String,
}

#[async_trait]
pub trait HitSearchAPIByKewWordUseCase {
    async fn hit_search_api(
        &self,
        params: RequestParams,
    ) -> Result<Vec<Tweet>, TwitterAPIAccessError>;
}

pub async fn execute<T>(
    api_access: T,
    params: RequestParams,
) -> Result<Vec<Tweet>, TwitterAPIAccessError>
where
    T: HitSearchAPIByKewWordUseCase,
{
    api_access.hit_search_api(params).await
}
