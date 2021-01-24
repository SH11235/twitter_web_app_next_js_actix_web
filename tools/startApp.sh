#!/bin/sh
cd `dirname $0`

if [ "$1" = "start" ]; then
	cd ../service/server/web
	if [ -n "$(lsof -t -i:8000)" ]; then
		kill -9 $(lsof -t -i:8000)
	fi
	mkdir -p ../log
	nohup cargo run > ../log/server.log &
	echo "server log output: service/server/log/server.log"

	cd ../../client/web
	if [ -n "$(lsof -t -i:3000)" ]; then
		kill -9 $(lsof -t -i:3000)
	fi
	mkdir -p ../log
	nohup npm run start > ../log/client.log &
	echo "client log output: service/client/log/client.log"
	echo "http://localhost:3000"
elif [ "$1" = "stop" ]; then
	if [ -n $(lsof -t -i:8000) ]; then
	kill -9 $(lsof -t -i:8000)
	fi
	if [ -n $(lsof -t -i:3000) ]; then
	kill -9 $(lsof -t -i:3000)
	fi
else
	echo "start: Restart Application, stop: Stop Application"
fi
