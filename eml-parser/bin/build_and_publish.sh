#!/bin/bash

set -e

test ${PWD##*/} = eml-parser

# Builds the app and the docker file, and then publishes it to docker hub.

readonly app_version=$(cat package.json | jq -r '.version')
readonly tag="porkbrain/newsletter:eml-parser-${app_version}"

# updates the current version stored in k8s spec
readonly capture_semver="[0-9]+\\.[0-9]+\\.[0-9]+(\\-[0-9A-Za-z\\.\\-]+)?"
readonly capture_img="porkbrain\\/newsletter\\:eml-parser"
readonly replace_img="porkbrain\\/newsletter:eml-parser"
sed -i -r \
    "/${capture_img}-${capture_semver}/s//${replace_img}-${app_version}/" \
    ../k8s/eml-parser.yml

npm i
npm run build
npm t

docker build -t "${tag}" .
docker push "${tag}"

