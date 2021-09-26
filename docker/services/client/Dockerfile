FROM node:14-alpine

WORKDIR /usr/src/twittersearch/client

RUN npm install

CMD npm run clean && npm ci && npm run start
