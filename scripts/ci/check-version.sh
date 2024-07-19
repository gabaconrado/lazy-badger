#!/usr/bin/env bash
#
# This script checks the version on the Cargo.toml file with the given tag

version=${1}

cargo_file="Cargo.toml"

if [ -z "${version}" ]; then
    echo "Missing version argument"
    exit 1
fi

if [ ! -f "${cargo_file}" ]; then
    echo "${cargo_file} not found"
    exit 2
fi

cargo_version=$(grep -Po '(?<=^version = ")[^"]*' "${cargo_file}")

if [ -z "${cargo_version}" ]; then
    echo "Version not found in '${cargo_file}'"
    exit 3
fi

if [ "${version}" != "${cargo_version}" ]; then
    echo "Version in '${cargo_file}|${cargo_version}' does not match given version '${version}'"
    exit 4
fi

echo "Version '${version}' is correct!"
