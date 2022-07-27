#!/bin/bash

set -ex

DOCKER_SWAGGER_URL="https://docs.docker.com/engine/api"
DOCKER_API_VERSION="v1.41"
DOCKER_SPEC_FILE="${DOCKER_API_VERSION}.yaml"
DOCKER_FULL_URL="${DOCKER_SWAGGER_URL}/${DOCKER_SPEC_FILE}"
RUSTGEN="https://git.wkepka.dev/wojtek/swagger-rustgen.git"
BUILD_DIR=build
BASE_DIR=$PWD

mkdir $BUILD_DIR || true

cd $BUILD_DIR
echo $PWD

curl -LO $DOCKER_FULL_URL

git clone $RUSTGEN || true
cd swagger-rustgen
cargo build --release
cd $BASE_DIR

cat base/models.rs > lib/src/models.rs

$BUILD_DIR/swagger-rustgen/target/release/swagger-rustgen generate models $BUILD_DIR/$DOCKER_SPEC_FILE >> lib/src/models.rs

cd lib

cargo fmt

