#!/bin/bash

set -e

test ${PWD##*/} = newsletter

# Runs all migrations against a file on the server storing the database. It's
# important that the migrations are idempotent. It's important that no other
# process writes into the database while migrations are running.

readonly node="gloss"
readonly host="192.168.0.125"
readonly pv_path="/var/local/newsletter"

echo "Syncing migrations dir"
rsync -av migrations "${node}@${host}":"${pv_path}"

for m in migrations/*.sql; do
    echo "Migrating ${m}"
    ssh "${node}@${host}" "sqlite3 ${pv_path}/database.db < ${pv_path}/${m}"
done

