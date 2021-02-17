# The testing loop I used

This command uses the latest version to annoate the dataset

``` sh
cargo run -- explain -i data/winequality/winequality.csv -r data/winequality/winequality-tsne-3d.csv data/winequality/annotations-3d.csv
cargo run -- explain -i data/winequality/winequality.csv -r data/winequality/winequality-tsne-2d.csv -b data/winequality/annotations-2d.csv
```

# Run the viewer
``` sh
# cube 2d pca
cargo run --release -- view -i data/cube/cube.csv --r2d data/cube/reduced-cube-pca-2d.csv

# cube 3d pca
cargo run --release -- view -i data/cube/hyper-cube.csv --r3d data/cube/reduced-hyper-cube-pca-3d.csv

# winequality 2d lamp
cargo run --release -- view -i data/winequality/winequality.csv --r2d data/winequality/winequality-lamp-2d.csv

# winequality 3d tsne
cargo run --release -- view -i data/winequality/winequality.csv --r3d data/winequality/winequality-tsne-3d.csv

# Both winequality
cargo run --release -- view -i data/winequality/winequality.csv --r2d data/winequality/winequality-lamp-2d.csv --r3d data/winequality/winequality-tsne-3d.csv
```