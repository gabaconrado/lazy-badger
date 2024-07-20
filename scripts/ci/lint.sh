#!/usr/bin/env bash
#
# This script lints the system
#

cargo clippy --color always --all-features --tests --examples --bins
