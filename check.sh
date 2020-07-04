#!/bin/sh
# check if code is even compilable
cargo check --all
# Run formater
cargo fmt --all -- --write-mode=diff
# run common mistake checker
cargo clippy --all --verbose -- -D warnings
# Run tests
cargo test --all
