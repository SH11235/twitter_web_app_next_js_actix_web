use crate::domain::entity::search_api::Tweet;
use crate::hit_api_utils::error::{TwitterAPIAccessError, UseCase};
use crate::hit_api_utils::setting::{bearer_token, search_api_url, twitter_base_url};
use crate::usecase::hit_twitter_api::hit_search_api;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct TwitterApiDriver {}

impl TwitterApiDriver {
    pub fn new() -> TwitterApiDriver {
        TwitterApiDriver {}
    }
}

impl Default for TwitterApiDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl UseCase for TwitterApiDriver {}

#[derive(Debug, Serialize, Deserialize)]
struct SearchAPIResult {
    search_metadata: Value,
    statuses: Vec<TweetInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TweetInfo {
    pub text: String,
    pub user: TweetUser,
    pub id_str: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetUser {
    pub name: String,
    pub screen_name: String,
    pub profile_image_url_https: String,
}

#[async_trait]
impl hit_search_api::HitSearchAPIByKewWordUseCase for TwitterApiDriver {
    async fn hit_search_api(
        &self,
        params: hit_search_api::RequestParams,
    ) -> Result<Vec<Tweet>, TwitterAPIAccessError> {
        let endpoint = search_api_url();
        let bearer_token = format!("Bearer {}", bearer_token());
        let res = reqwest::Client::new()
            .get(&endpoint)
            .query(&[
                ("q", &params.q),
                ("count", &params.count),
                ("result_type", &params.result_type),
                ("max_id", &params.max_id),
            ])
            .header("authorization", bearer_token)
            .send()
            .await
            .map_err(|e| TwitterAPIAccessError::InternalErrorWithMessage(e.to_string()))?;
        let res_json: SearchAPIResult = res
            .json()
            .await
            .map_err(|e| TwitterAPIAccessError::InternalErrorWithMessage(e.to_string()))?;

        let tweet_info_vec = res_json.statuses;
        let twitter_base_url = twitter_base_url();
        let ret: Vec<Tweet> = tweet_info_vec
            .iter()
            .map(|tweet_info| {
                let user_link = format!(
                    "{}{}{}",
                    &twitter_base_url, "/", tweet_info.user.screen_name
                );
                let tweet_link = format!("{}{}{}", user_link, "/status/", tweet_info.id_str);
                Tweet {
                    tweet_id: tweet_info.id_str.to_string(),
                    text: tweet_info.text.to_string(),
                    tweet_link,
                    user_link,
                    tweet_time: tweet_info.created_at.to_string(),
                    user_name: tweet_info.user.name.to_string(),
                    screen_name: tweet_info.user.screen_name.to_string(),
                    profile_image_url: tweet_info.user.profile_image_url_https.to_string(),
                }
            })
            .collect();
        Ok(ret)
    }
}
