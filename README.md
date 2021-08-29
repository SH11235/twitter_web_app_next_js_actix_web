# twitter_api_rust

## 環境

- node.js 14.4
- rustc 1.54
- cargo 1.54
- rustup 1.24.3
- postgres 13.2

## docker 環境でのアプリ起動

### docker. docker-compose のインストール

公式ドキュメントを見る。

- https://docs.docker.com/get-docker/
- https://docs.docker.jp/compose/install.html#linux-compose

### node.js のインストール

- Ubuntu 20.04<br>
  apt でインストール<br>
  https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-20-04-ja

- 複数バージョンの node を使う場合<br>
  anyenv などを利用して入れる<br>
  https://qiita.com/KZ-taran/items/f25a7d608e8ca9b258bf

- パッケージのインストール<br>
  cd service/web/client && npm install

### ログ用のディレクトリ

- mkdir service/web/client/log
- mkdir service/web/server/log

### docker を使ってのアプリ起動

- （再）起動：tools/startDocker.sh start
- 停止：tools/startDocker.sh stop

### コンテナに入るコマンド

- client：cd docker && docker-compose exec client
- server：cd docker && docker-compose exec server

## docker を使わない場合のアプリ起動・停止

- （再）起動：tools/startApp.sh start
- 停止：tools/startApp.sh stop

## diesel

postgres が起動した状態で

- cd service/web/server
- diesel migration run

## 動作確認

### server

- キーワード「keyword」で検索した結果
  http://localhost:8000/twitter_search?q=keyword&type=mixed
  json が返ってきていれば動作 OK

### client

- 検索画面
  http://localhost:3000
