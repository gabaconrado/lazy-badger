#!/usr/bin/env bash
#
# This script lints the system
#

cargo clippy --all-features --tests --examples --bins
