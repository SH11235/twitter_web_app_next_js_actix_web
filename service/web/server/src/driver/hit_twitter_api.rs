use crate::domain::entity::search_api::Tweet;
use crate::hit_api_utils::error::{TwitterAPIAccessError, UseCase};
use crate::hit_api_utils::setting::{
    bearer_token, get_request_num, search_api_url, twitter_base_url, MAX_TWEET_NUM,
};
use crate::usecase::hit_twitter_api::hit_search_api;
use async_trait::async_trait;
use qstring::QString;
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
        let count_param = params.count.parse().unwrap();
        let request_num: i32 = get_request_num(count_param);
        let mut max_id = "0".to_string();
        let mut values: Vec<Tweet> = vec![];
        for idx in 0..request_num {
            let count = if idx == request_num - 1 {
                if count_param % MAX_TWEET_NUM != 0 {
                    (count_param % MAX_TWEET_NUM).to_string()
                } else {
                    MAX_TWEET_NUM.to_string()
                }
            } else {
                MAX_TWEET_NUM.to_string()
            };
            let res = reqwest::Client::new()
                .get(&endpoint)
                .query(&[
                    ("q", &params.q),
                    ("count", &count),
                    ("result_type", &params.result_type),
                    ("max_id", &max_id),
                ])
                .header("authorization", &bearer_token)
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
            values.extend(ret);
            let next_results = res_json.search_metadata["next_results"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let qs = QString::from(next_results.as_str());
            max_id = qs.get("max_id").unwrap_or("0").to_string();
        }

        Ok(values)
    }
}
