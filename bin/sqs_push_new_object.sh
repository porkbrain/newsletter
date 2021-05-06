#!/bin/bash

# Given a list of object keys in stdin, a bucket name and a queue name, it
# pushes all objects into the sqs just as if they were recently created.
#
# Example usage:
#
# cat keys.txt | ./bin/sqs_push_new_object.sh "mailmevouchers" "new_email"
#

set -e

readonly region="eu-west-1"

function queue_url() {
    aws sqs get-queue-url \
        --queue-name "${queue_name}" \
        --region "${region}" |
        jq -r '.QueueUrl' ||
        exit $?
}

readonly bucket_name="${1}"
readonly queue_name="${2}"

if [ -z "${bucket_name}" ]; then
    echo "Bucket name must be provided"
    exit 1
fi

if [ -z "${queue_name}" ]; then
    echo "Queue name must be provided"
    exit 1
fi

echo "Fetching queue url for ${queue_name}"
readonly queue_url="$(queue_url "${queue_name}")"
if [ ! $? -eq 0 ]; then
    echo "Cannot get queue url"
    exit 1
fi

function send_msg() {
    local body=$1

    aws sqs send-message \
        --region "${region}" \
        --queue-url "${queue_url}" \
        --message-body "${body}"
}

echo "Pushing new object insertion events from bucket ${bucket_name} into SQS
${queue_url} for:"

while read object_key
do
    send_msg \
        "{\"Records\":[{\"awsRegion\":\"${region}\",\
        \"s3\":{\"bucket\":{\"name\":\"${bucket_name}\"},\
        \"object\":{\"key\":\"${object_key}\"}}}]}"
done < "${3:-/dev/stdin}"

