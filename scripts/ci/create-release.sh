#!/usr/bin/env bash
#
# This script uses the GH CLI to create a release
#

tag="${1}"

if [ -z "${tag}" ]; then
    echo "Missing tag parameter"
    exit 1
fi

gh release create "${tag}" --generate-notes --verify-tag

