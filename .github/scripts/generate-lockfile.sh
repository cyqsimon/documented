#!/usr/bin/env bash

# This script generates the lockfile for the current toolchain.
# It is only necessary because the MSRV is currently lower than 1.84.0,
# which is the minimum needed for resolver v3.
# If and when the MSRV is raised to or above 1.84.0, this script can be
# replaced by a simple `cargo generate-lockfile`.

set -e

RESOLVER_V3_MIN_VERSION='1.84.0'
VERSION="$(cargo --version | cut -d' ' -f2)"

LOWER_VERSION="$(echo -e "$RESOLVER_V3_MIN_VERSION\n$VERSION" | sort --version-sort | head -n1)"

if [[ "$LOWER_VERSION" == "$RESOLVER_V3_MIN_VERSION" ]]; then
    cargo generate-lockfile
else
    # must declare workspace resolver, or it defaults to v1
    sed -Ei '/resolver ?=/ s/3/2/' Cargo.toml

    cargo generate-lockfile

    # manually downgrade dependencies to compatible versions
    cargo update --package unicode-segmentation --precise 1.12.0
fi
