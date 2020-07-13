# Useful to checks on the code before pushing.

What is required before running the code (this omits the nvvc + gcc-7 section)
- export PATH="$CARGO_HOME/bin:$PATH"
- curl https://sh.rustup.rs -sSf | sh -s -- -y  --default-toolchain stable
- rustup component add rustfmt-preview
- rustup --version
- rustc --version
- cargo --version

script:
- cargo check --all --verbose # check if code is even compilable
- cargo fmt --all --verbose -- --write-mode=diff # Run formater
- cargo clippy --all --verbose -- -D warnings # run common mistake checker
- cargo test --all --verbose # Run tests