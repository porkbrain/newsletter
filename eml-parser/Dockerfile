FROM node:12

WORKDIR /usr/src/app

# install prod deps
COPY package*.json ./
RUN npm ci --only=production

# it's important that you build the app first
COPY dist dist
RUN test -f dist/main.js

CMD [ "node", "." ]

