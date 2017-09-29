#!/usr/bin/env bash

MODE="--release"
if [ "$1" = "--debug" ]; then
    shift
    MODE=
fi

time RUST_MIN_STACK=16777216 cargo +nightly run --bin demo $MODE "$@"

