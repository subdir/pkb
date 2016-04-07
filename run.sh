#!/bin/bash

set -e
set -o pipefail

docker build \
    --tag=rust_dev \
    --rm=true \
    docker

home=/home/dev

docker run \
    --rm \
    --tty \
    --interactive \
    --volume="$PWD:$home:rw" \
    --workdir=$home \
    --env=HOME="$home" \
    --env=TARGET_UID=$(id -u) \
    --env=TARGET_GID=$(id -g) \
    rust_dev \
    "$@"
