#!/bin/bash

set -ex

mvn clean compiler:compile generate-resources

cd ./target/gen

cargo fmt

sed -i -r "s/(GenericResources)_inner/\1Inner/g" src/models.rs

