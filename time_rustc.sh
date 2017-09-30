#!/usr/bin/env bash

PREFIX=
for arg in "$@"; do
    if [[ "$arg" == *.rs ]]; then
        echo "compiling $arg"
        PREFIX=time
    fi
done

$PREFIX rustc "$@"

