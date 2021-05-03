#!/bin/bash


readonly bucket_name="${1}"

if [ -z "${bucket_name}" ]; then
    echo "Bucket name must be provided"
    exit 1
fi

readonly now_date="$(date +"%Y-%m-%d %H:%M:%S")"
readonly since_date="${2:-$now_date}"
readonly to_date="${3}"

readonly list="$(aws s3 ls ${bucket_name})"

while IFS= read -r line; do
    updated_at="${line::19}"
    object_key="${line: - 40}"

    if [[ "${updated_at}" > "${since_date}" ]]; then
        if [[ "${to_date}" == "" || "${updated_at}" < "${to_date}" ]]; then
            echo "${object_key}"
        fi
    fi
done <<< "$list"



