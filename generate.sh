#!/bin/bash
set -e
mkdir -p tags

# The matrix
cargo run -- --include Pyenv39
cargo run -- --include Pyenv39 --include Stripped

# Build all tags
for dockerfile in tags/Dockerfile.*; do
    filename=$(basename -- "$dockerfile")
    tag="${filename##*.}"
    docker build --tag archmatrix:$tag --file $dockerfile context
done