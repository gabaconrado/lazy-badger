#!/usr/bin/env bash
#
# This script runs all validations that are run in CI development workflow at once
#

set -e -o pipefail

./scripts/ci/build.sh
./scripts/ci/lint.sh
./scripts/ci/test.sh
