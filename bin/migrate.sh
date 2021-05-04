#!/bin/bash

set -e

test ${PWD##*/} = newsletter

# Runs all migrations against a file on the server storing the database. It's
# important that the migrations are idempotent. It's important that no other
# process writes into the database while migrations are running.

readonly node="gloss"
readonly host="doma"
readonly port="${1}"
readonly pv_path="/var/local/newsletter"

if [ -z ${port} ]; then
    echo "gloss@doma ssh port must be provided"
    exit 1
fi

echo "Syncing migrations dir"
rsync -av -e "ssh -p ${port}" migrations "${node}@${host}":"${pv_path}"

for m in migrations/*.sql; do
    echo "Migrating ${m}"
    ssh -p "${port}" "${node}@${host}" "sqlite3 ${pv_path}/database.db < ${pv_path}/${m}"
done

