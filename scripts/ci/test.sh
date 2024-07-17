#!/usr/bin/env bash
#
# This script lints the system
#
# It checks both formatting and rust + clippy linting rules
#

cargo fmt --check
cargo test --all-features
