# Pointctl

A rust tool used to interact with point clouds. It can be used to generate, subsample and display clouds.

## Running the CLI tool

``` sh
# installing the release version to `~/.cargo/bin`
cargo install --path .
pointctl --help

# alternatively you can run the dev version without installing
cargo run -- --help
```

## Running the cuda examples:

``` sh
cargo run --example cuda-add
cargo run --example cuda-hello
```