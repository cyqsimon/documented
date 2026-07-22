#!/usr/bin/env bash

# This script generates the lockfile for the current toolchain.
# It is only necessary because the MSRV is currently lower than 1.84.0,
# which is the minimum needed for resolver v3.
# If and when the MSRV is raised to or above 1.84.0, this script can be
# replaced by a simple `cargo generate-lockfile`.

set -e

# allow overriding cargo binary
: "${CARGO_BIN:=cargo}"
echo "\`$CARGO_BIN --version\`: $($CARGO_BIN --version)"
: "${CARGO_STABLE_BIN:=cargo +stable}"
echo "\`$CARGO_STABLE_BIN --version\`: $($CARGO_STABLE_BIN --version)"

RESOLVER_V3_MIN_VERSION='1.84.0'
VERSION="$($CARGO_BIN --version | cut -d' ' -f2)"

LOWER_VERSION="$(echo -e "$RESOLVER_V3_MIN_VERSION\n$VERSION" | sort --version-sort | head -n1)"

if [[ "$LOWER_VERSION" == "$RESOLVER_V3_MIN_VERSION" ]]; then
    $CARGO_BIN generate-lockfile
else
    # generate lockfile with resolver v3 using stable toolchain
    $CARGO_STABLE_BIN generate-lockfile

    # then make it MSRV-compatible
    # must declare workspace resolver, or it defaults to v1
    sed -Ei '/resolver ?=/ s/3/2/' Cargo.toml
fi
