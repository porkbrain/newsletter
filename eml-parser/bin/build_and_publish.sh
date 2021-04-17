#!/bin/bash

## Builds the app and the docker file, and then publishes it to docker hub.

set -e

readonly app_version=$(cat package.json | jq -r '.version')
readonly tag="porkbrain/newsletter:eml-parser-${app_version}"

npm i
npm run build
npm t

docker build -t "${tag}" .
docker push "${tag}"

