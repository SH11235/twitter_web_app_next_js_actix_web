FROM node:14-alpine

WORKDIR /usr/src/twittersearch/client/web

RUN mkdir ./log

CMD npm run clean && npm ci && npm run start > ./log/client.log && ash
