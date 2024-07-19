#!/usr/bin/env bash
#
# This script creates a tag and pushes to the repository
#

MAIN_BRANCH="main"

tag="${1}"

if [ -z "${tag}" ]; then
    echo "Missing tag parameter"
fi

current_branch="$(git branch --show-current)"

if [ "${current_branch}" != "${MAIN_BRANCH}" ]; then
    echo "Tags must be created on the '${MAIN_BRANCH}'. Current is '${current_branch}'"
fi

git tag "${tag}" -a && git push origin "${tag}"
