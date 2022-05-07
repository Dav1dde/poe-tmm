#!/usr/bin/env bash

set -e -o pipefail

if [[ "$1" == "--release" ]]; then
    echo "Building in --release mode"
    WORKER_BUILD_ARGS="--release"
elif [[ "$1" == "--dev" ]]; then
    echo "Building in --dev mode"
    WORKER_BUILD_ARGS="--dev -- --features debug"
else
    echo "expected --release or --dev"
    exit 1
fi

cd worker
worker-build $WORKER_BUILD_ARGS

cd ..
