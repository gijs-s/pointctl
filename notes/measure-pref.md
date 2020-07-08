# Quick note on how to measure performance

First ensure you have perf installed, then add the following to cargo.toml:

``` toml
[profile.release]
debug = true
```

after you can build a release binary with the profiling attributes (`cargo build --release`). After you can start measuring performance


``` sh
# Run the latest build with perf using a full call graph
perf record -g --call-graph=dwarf ./target/release/poinctl
# Generate flame graph using
# https://github.com/brendangregg/FlameGraph and https://github.com/Yamakaky/rust-unmangle/blob/master/rust-unmangle
perf script | ../FlameGraph/stackcollapse-perf.pl | ../FlameGraph/rust-unmangle | ../FlameGraph/flamegraph.pl > flame.svg
