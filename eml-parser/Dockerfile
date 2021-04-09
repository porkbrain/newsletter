FROM node:12

ENV BIN_DIRECTORY /usr/bin/mailmevouchers
WORKDIR "${BIN_DIRECTORY}"

# dependencies and built files should be everything we need
COPY node_modules node_modules
COPY dist dist

CMD node dist/index.js
