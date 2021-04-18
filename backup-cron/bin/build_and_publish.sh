#!/bin/bash

set -e

test ${PWD##*/} = backup-cron

# Builds and publishes docker image for running sql backups.

readonly tag="porkbrain/s3-sqlite-backup"

docker build -t "${tag}" .
docker push "${tag}"

