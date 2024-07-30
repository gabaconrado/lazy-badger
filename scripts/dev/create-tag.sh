#!/usr/bin/env bash
#
# This script creates a tag and pushes to the repository
#

MAIN_BRANCH="main"

tag="${1}"

if [ -z "${tag}" ]; then
    echo "Missing tag parameter"
    exit 1
fi

./scripts/ci/check-version.sh "${tag}"

current_branch="$(git branch --show-current)"

if [ "${current_branch}" != "${MAIN_BRANCH}" ]; then
    echo "Tags must be created on the '${MAIN_BRANCH}'. Current is '${current_branch}'"
fi

git tag "${tag}" && git push origin "${tag}"
