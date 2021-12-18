# twitter_api_rust

## 環境

- node.js 16.13.1
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

- node のバージョン管理<br>
  volta を使用<br>
  https://volta.sh/

- パッケージのインストール<br>
  cd service/web/client && npm install

### PostgreSQL

docker を使うのを推奨
cd docker && docker-compose up -d postres

### ログ用のディレクトリ

- mkdir service/web/client/log
- mkdir service/web/server/log

### docker を使ってのアプリ起動

- （再）起動：tools/startDocker.sh start
- 停止：tools/startDocker.sh stop

### コンテナに入るコマンド

- client：cd docker && docker-compose exec client
- server：cd docker && docker-compose exec server
- postres：cd docker && docker-compose exec postres

## docker を使わない場合のアプリ起動・停止

- （再）起動：tools/startApp.sh start
- 停止：tools/startApp.sh stop

## diesel

- cargo install diesel_cli
- cd service/web/server
  postgres が起動した状態で migration する
- diesel migration run

## 動作確認

### server

- キーワード「keyword」で検索した結果
  http://localhost:8000/twitter_search?q=keyword&type=mixed
  json が返ってきていれば動作 OK

### client

- 検索画面
  http://localhost:3000
