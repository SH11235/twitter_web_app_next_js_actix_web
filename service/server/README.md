# twitter_api_rust

`cargo run`
http://localhost:8000/twitter_search?q=keyword&count=10 <br>
で動作確認。<br>
Twitter の search API の結果が json で返ってくる。<br>
URL パラメータで以下の 2 つを指定する。<br>
q：検索キーワード<br>
count：検索件数

## json を post する例

curl -X POST -H "Content-Type: application/json" -d \
'{"text": "テスト用", "tweet_link": "String", "user_link": "String", "tweet_time": "String", "user_name": "String", "screen_name": "String", "profile_image_url": "String"}' \
localhost:8000/tweets/
