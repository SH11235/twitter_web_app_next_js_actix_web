# twitter_api_rust

## 環境
- node.js 14.4
- rustc 1.49
- cargo 1.49
- rustup 1.23.1

## docker環境でのアプリ起動
### docker. docker-composeのインストール
公式ドキュメントを見る。
- https://docs.docker.com/get-docker/
- https://docs.docker.jp/compose/install.html#linux-compose

### node.jsのインストール
- Ubuntu 20.04
aptでインストール
https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-20-04-ja

- 複数バージョンのnodeを使う場合
anyenvなどを利用して入れる
https://qiita.com/KZ-taran/items/f25a7d608e8ca9b258bf

- パッケージのインストール
cd service/client/web && npm install

### ログ用のディレクトリを作る
- mkdir service/client/web/log
- mkdir service/server/web/log

### docker起動
- （再）起動：tools/startDocker.sh start
- 停止：tools/startDocker.sh stop

## （dockerを使わない場合の）アプリ起動・停止
- （再）起動：tools/startApp.sh start
- 停止：tools/startApp.sh stop

## 動作確認
### server
- キーワード「keyword」で検索した結果
http://localhost:8000/twitter_search?q=keyword&type=mixed
jsonが返ってきていれば動作OK

### client
- 検索画面
http://localhost:3000
