#!/bin/bash
set -e
rm -rf tags
mkdir tags

# The matrix
cargo run -- --include Pyenv39
cargo run -- --include Pyenv39 --include A
cargo run -- --include Pyenv39 --include Stripped
cargo run -- --include Pyenv310
cargo run -- --include Pyenv310 --include A
cargo run -- --include Pyenv310 --include Stripped
cargo run -- --include Tf12
cargo run -- --include Aws
cargo run -- --include Tf12 --include Aws

# Build all tags
for dockerfile in tags/Dockerfile.*; do
    filename=$(basename -- "$dockerfile")
    tag="${filename##*.}"
    docker build --tag "archmatrix:$tag" --file "$dockerfile" context
done