#!/bin/bash

set -e

file="${1}"
lang="${2}"

if [ ! -f "${file}" ]; then
    echo "'${1}' isn't a file"
    exit 1
fi

if [ -f "${file}.tmp" ]; then
    echo "Delete '${file}.tmp' backup first"
    exit 1
fi

if [ "${lang}" != "en" ]; then
    echo "Only English is supported"
    exit 1
fi      

# Strips non ascii or invisible characters
# (https://stackoverflow.com/a/20890052/5093093)
# (https://stackoverflow.com/a/33670925/5093093)
LANG=C sed -i 's/[^\d32-\d126]/ /g' "${file}"

# remove common tags
sed -i -r 's/<link>/ /g' "${file}"
sed -i -r 's/<\/link>/ /g' "${file}"

# strip extra white spaces and quotes which start or end line
sed -i -r 's/\s+/ /g' "${file}"
sed -i -r 's/"$//g' "${file}"
sed -i -r 's/^"//g' "${file}"
sed -i -r 's/^\s+//g' "${file}"
sed -i -r 's/\s+$//g' "${file}"

# remove any line which has one or less spaces and then all duplicates
cat "${file}" | grep -E '(.*\s.*\s.*)' | sort | uniq > "${file}.tmp"
mv "${file}.tmp" "${file}"

