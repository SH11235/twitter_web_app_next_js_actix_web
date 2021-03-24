-- Your SQL goes here
CREATE TABLE tweets (
  id SERIAL PRIMARY KEY,
  text TEXT NOT NULL,
  tweetLink VARCHAR NOT NULL,
  userLink VARCHAR NOT NULL,
  tweetTime VARCHAR NOT NULL,
  userName VARCHAR NOT NULL,
  screenName VARCHAR NOT NULL,
  profileImageUrl VARCHAR NOT NULL
)
