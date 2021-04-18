#!/bin/bash

set -e

if [ -z "${DATABASE_PATH}" ]; then
    echo "DATABASE_PATH env must be provided"
    exit 1
fi

if [ -z "${BUCKET_NAME}" ]; then
    echo "BUCKET_NAME env must be provided"
    exit 1
fi

readonly now=$(date "+%Y-%d-%m_%H-%M-%S" -u)
readonly backup_path="/var/local/backups/${now}.db"

if [ -z "${PREFIX}" ]; then
    object_path="${now}.db"
else
    object_path="${PREFIX}/${now}.db"
fi

echo "Performing backup to ${backup_path}..."
sqlite3 "${DATABASE_PATH}" ".backup ${backup_path}"

echo "Copying backup to bucket ${BUCKET_NAME}..."
aws s3 cp "${backup_path}" "s3://${BUCKET_NAME}/${object_path}"

