#!/bin/bash
ROOT_DIR="$(cd $(dirname $0)/..; pwd)";

if [ "$1" = "start" ]; then
	cd $ROOT_DIR/docker
	docker-compose stop
	docker-compose up -d
	echo "server log output: service/server/log/server.log"
	echo "client log output: service/client/log/client.log"
	echo "http://localhost:3000"
elif [ "$1" = "stop" ]; then
	cd $ROOT_DIR/docker
	docker-compose stop
else
	echo "start: Restart Container, stop: Stop Container"
fi
