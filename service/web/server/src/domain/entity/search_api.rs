use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub tweet_id: String,
    pub text: String,
    pub tweet_link: String,
    pub user_link: String,
    pub tweet_time: String,
    pub user_name: String,
    pub screen_name: String,
    pub profile_image_url: String,
}

// pub struct TweetIntoIterator {
//     tweet: Tweet,
//     index: usize,
// }

// impl IntoIterator for Tweet {
//     type Item = String;
//     type IntoIter = TweetIntoIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         TweetIntoIterator {
//             tweet: self,
//             index: 0,
//         }
//     }
// }

// impl Iterator for TweetIntoIterator {
//     type Item = String;
//     fn next(&mut self) -> Option<String> {
//         let result = match self.index {
//             0 => self.tweet.tweet_id,
//             1 => self.tweet.text,
//             2 => self.tweet.tweet_link,
//             3 => self.tweet.user_link,
//             4 => self.tweet.tweet_time,
//             5 => self.tweet.user_name,
//             6 => self.tweet.screen_name,
//             7 => self.tweet.profile_image_url,
//             _ => return None,
//         };
//         self.index += 1;
//         Some(result)
//     }
// }
