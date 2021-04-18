#!/bin/bash

set -e

test ${PWD##*/} = prtsc

# Builds the app and the docker file, and then publishes it to docker hub.

readonly app_version="$(
    cat Cargo.toml |
    grep 'name = "prtsc"' -A 1 |
    sed -n 's/^version = "\(.*\)"$/\1/p'
)"
readonly tag="porkbrain/newsletter:prtsc-${app_version}"

# updates the current version stored in k8s spec
# TODO: move to build.rs
readonly capture_semver="[0-9]+\\.[0-9]+\\.[0-9]+(\\-[0-9A-Za-z\\.\\-]+)?"
readonly capture_img="porkbrain\\/newsletter\\:prtsc"
readonly replace_img="porkbrain\\/newsletter:prtsc"
sed -i -r \
    "/${capture_img}-${capture_semver}/s//${replace_img}-${app_version}/" \
    ../k8s/prtsc.yml

cargo test
cargo build --release

cp ../target/release/prtsc prtsc.bin
docker build -t "${tag}" .
rm prtsc.bin

docker push "${tag}"

