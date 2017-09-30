#!/usr/bin/env bash

MODE="--release"
if [ "$1" = "--debug" ]; then
    shift
    MODE=
fi

RUST_MIN_STACK=16777216 cargo +nightly run --bin demo $MODE "$@"

