FROM node:12

WORKDIR /usr/src/app

RUN apt-get update -y && apt install sqlite3 -y
RUN sqlite3 --version

# install prod deps
COPY package*.json ./
RUN npm ci --only=production

# it's important that you build the app first
COPY src src

CMD [ "node", "." ]

