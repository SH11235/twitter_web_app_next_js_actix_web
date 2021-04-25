FROM node:14-alpine

WORKDIR /usr/src/twittersearch/client/web

RUN npm install

CMD npm run clean && npm ci && npm run start
