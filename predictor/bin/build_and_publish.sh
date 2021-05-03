#!/bin/bash

set -e

test ${PWD##*/} = predictor

# Builds the app and the docker file, and then publishes it to docker hub.

readonly app_version="$(
    cat Cargo.toml |
    grep 'name = "predictor"' -A 1 |
    sed -n 's/^version = "\(.*\)"$/\1/p'
)"
readonly tag="porkbrain/newsletter:predictor-${app_version}"

# updates the current version stored in k8s spec
# TODO: move to build.rs
readonly capture_semver="[0-9]+\\.[0-9]+\\.[0-9]+(\\-[0-9A-Za-z\\.\\-]+)?"
readonly capture_img="porkbrain\\/newsletter\\:predictor"
readonly replace_img="porkbrain\\/newsletter:predictor"
sed -i -r \
    "/${capture_img}-${capture_semver}/s//${replace_img}-${app_version}/" \
    ../k8s/predictor.yml

cargo test
cargo build --release

cp ../target/release/predictor predictor.bin
docker build -t "${tag}" .
rm predictor.bin

read -p "Do you wish to push ${tag}? Y/n" -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
    docker push "${tag}"
fi

