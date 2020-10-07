#!/bin/sh
echo "\033[1;36mRunning Cargo check\033[0m"
echo ""
cargo check --all

# Run formater check
echo ""
echo "\033[1;36mRunning Cargo format\033[0m"
echo ""
cargo fmt --all -- --check

# run common mistake checker
echo ""
echo "\033[1;36mRunning Cargo clippy\033[0m"
echo ""
cargo clippy --all --verbose -- -D warnings

# Run tests
echo ""
echo "\033[1;36mRunning the tests\033[0m"
echo ""
cargo test --all
