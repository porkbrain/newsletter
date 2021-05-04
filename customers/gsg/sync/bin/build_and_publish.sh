#!/bin/bash

set -e

test ${PWD##*/} = sync

# Builds the app and the docker file, and then publishes it to docker hub.

readonly app_version=$(cat package.json | jq -r '.version')
readonly tag="porkbrain/newsletter:gsg-sync-${app_version}"

# updates the current version stored in k8s spec
readonly capture_semver="[0-9]+\\.[0-9]+\\.[0-9]+(\\-[0-9A-Za-z\\.\\-]+)?"
readonly capture_img="porkbrain\\/newsletter\\:gsg-sync"
readonly replace_img="porkbrain\\/newsletter:gsg-sync"
sed -i -r \
    "/${capture_img}-${capture_semver}/s//${replace_img}-${app_version}/" \
    ../../../k8s/gsg-sync.yml

npm i

docker build -t "${tag}" .
docker push "${tag}"

