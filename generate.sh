#!/bin/bash
mkdir -p tags

# The matrix
cargo run -- --include Pyenv39

# Build all tags
for dockerfile in tags/Dockerfile.*; do
    docker build --file $dockerfile context
done