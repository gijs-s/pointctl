perf record -g --call-graph dwarf -- ./target/release/pointctl
cp perf.data ../FlameGraph
cd ../FlameGraph
perf script | stackcollapse-perf.pl | rust-unmangle | flamegraph.pl > flame.svg