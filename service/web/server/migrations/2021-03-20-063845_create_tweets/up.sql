-- Your SQL goes here
CREATE TABLE tweets (
  id SERIAL PRIMARY KEY,
  tweet_id VARCHAR NOT NULL,
  text TEXT NOT NULL,
  tweet_link VARCHAR NOT NULL,
  user_link VARCHAR NOT NULL,
  tweet_time VARCHAR NOT NULL,
  user_name VARCHAR NOT NULL,
  screen_name VARCHAR NOT NULL,
  profile_image_url VARCHAR NOT NULL
)
