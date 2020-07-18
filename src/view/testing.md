# The repl loop I used

``` sh
cargo run -- explain -i data/winequality/winequality.csv -r data/winequality/winequality-tsne-3d.csv data/winequality/annotations-3d.csv
cargo run -- explain -i data/winequality/winequality.csv -r data/winequality/winequality-tsne-2d.csv -b data/winequality/annotations-2d.csv

# Run the viewer
cargo run -- view -i data/winequality/winequality.csv -r data/winequality/winequality-tsne-3d.csv -a data/winequality/annotations-3d.csv -x data/winequality/winequality-tsne-2d.csv -b data/winequality/annotations-2d.csv
```