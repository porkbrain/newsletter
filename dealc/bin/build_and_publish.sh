#!/bin/bash

set -e

test ${PWD##*/} = dealc

# Builds the app and the docker file, and then publishes it to docker hub.

readonly app_version=$(cat version.txt)
readonly tag="porkbrain/newsletter:dealc-${app_version}"

# updates the current version stored in k8s spec
readonly capture_semver="[0-9]+\\.[0-9]+\\.[0-9]+(\\-[0-9A-Za-z\\.\\-]+)?"
readonly capture_img="porkbrain\\/newsletter\\:dealc"
readonly replace_img="porkbrain\\/newsletter:dealc"
sed -i -r \
    "/${capture_img}-${capture_semver}/s//${replace_img}-${app_version}/" \
    ../k8s/dealc.yml

docker build -t "${tag}" .
docker push "${tag}"

