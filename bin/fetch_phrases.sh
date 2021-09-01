#!/bin/bash

# Given a list of object keys in stdin and a bucket name, it fetches prediction
# documents from the S3 and extracts all phrases in each object.
#
# Example usage:
#
# cat keys.txt | ./bin/fetch_phrases.sh "newsletter-prediction-g47h"
#

readonly region="eu-west-1"
readonly bucket_name="${1}"

if [ -z "${bucket_name}" ]; then
    echo "Bucket name must be provided"
    exit 1
fi

readonly tmp_dir_name="$(tr -dc A-Za-z0-9 </dev/urandom | head -c 8)"
readonly tmp_dir="/tmp/${tmp_dir_name}"
mkdir -p "${tmp_dir}"

while read object_key
do
    aws s3 cp "s3://${bucket_name}/${object_key}" "${tmp_dir}"

    cat "${tmp_dir}/${object_key}" \
        | jq '.[] | .text' -cr \
        >> "${tmp_dir_name}.txt"
done < "${3:-/dev/stdin}"

rm -rf "${tmp_dir}"

echo "All lines written to ./${tmp_dir_name}.txt"
