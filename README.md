# Pointctl

A tool used to interact and understand dimensionality reduced data. It can be used to generate, view and explain point clouds!

# Running the CLI tool

First step is to setup all the dependencies as specified in the [section on dependencies](#deps). After this you can simply run the following commands:

``` sh
# installing the release version to `~/.cargo/bin`
cargo install --path .
pointctl --help

# alternatively you can run the dev version without installing. Note that the explanation mechanisms will be very slow.
cargo run -- --help

# Example call with included dataset (running a release build)
pointctl view --input data/Wine/winequality-src-both.csv --r3d data/Wine/winequality-tsne-3d.csv  --r2d data/Wine/winequality-lamp-both-2d.csv
```

In this example call the `-i` is the original nd data and `--r3d` is the data projected to 3d (same for `--r2d` and `2d`). The files need to be in a `csv` file with a header and `;` as seperator. During my testing I often used the `to_csv` function from pythons `pandas` library, python makes it easy to load in many formats and then you can simply call `df.to_csv('file.csv', header=True, index=False, sep=';')` to store it in the correct format.

# <a name="deps"></a> System dependencies

## Rust

Rust can be installed with one simple command:

``` sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you do not feel comfortable piping a website into shell I completely understand, read up about how to do manual installs here: https://www.rust-lang.org/tools/install. Do not forget to add `~/.cargo/bin` to your path variable!

For the rendering library you will also need `libxcb-xinput-dev` and `libxcb1-dev` if you are running on linux. For the linear algebra I use `lapack` which relies on `openblas` by default, this requires `gfortran` and `libgfortran-{7,8,9}-dev` to be compiled in the background.

## Python

This is not needed for the main program, it is used for quick debugging and prepoccesing of datasets. If you do want to make use of this offline part of the pipeline you wil need python 3.7/3.8 for the scripts and `pipenv` for the python packages. To use this you might need to install the `deadsnakes` ppa which includes if you are not on Ubuntu 20.04. Instruction on can be found here: https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa. After you can install `pipenv` according to this guide: https://pipenv-fork.readthedocs.io/en/latest/install.html#installing-pipenv.

When this installed running `pipenv sync` in the `jupyter/` will create a virtual python environment that contains all the dependencies. This directory is currently used for some quick protyping and to call ScikitLearn functions. Running the `./run.sh` script will start a jupyter interactive python site that you can use for this.

## Datasets

Included with this repo there are 8 datasets each with numerous projections into 2D and 3D. To fetch these you will need to install git lfs by following this guide: https://docs.github.com/en/github/managing-large-files/installing-git-large-file-storage.

After that you can run these commands to download all the datasets:

```sh
# Install the pre and post commit lfs hooks
git lfs install
# Manually fetch the files once
git lfs pull
```

For information on each dataset view the `readme.md` file in the `data/` folder!

## Quick navigation (Optional)

Because in this repository we are working with many datasets and a pretty nested storage structure I have a few tools that can greatly improve the quality of life while working with in it.

### Fuzzy command line file finder

First and formost is `fzf`, an interactive Unix filter for command-line that can be used with any list; files, command history, processes, hostnames, bookmarks, git commits, etc... This is useful when searching the entire `data` folder which has heavy nesting. To use it simply install and start the filter by pressing `Ctrl+T`.

``` sh
# Installing instructions
git clone --depth 1 https://github.com/junegunn/fzf.git ~/.fzf
~/.fzf/install
```

Source code: https://github.com/junegunn/fzf

### Nicer file viewer from command line

Secondly we have `bat`, this basicly a `cat` clone with very nice syntax highlighting. Highly recommend using this to quickly navigate giant files efficiently. Installed using `apt install bat` on ubuntu 19.10 and later or from source using `cargo install --locked bat`.

Source code: https://github.com/sharkdp/bat

# Project structure

The project is build up from multiple separate modules and might seems like a mess when first looking at the folder structure. Because of this I would like to elaborate quickly on what is stored where.

## Rust

The rust project itself is all contained in the `src` folder. Here we have the following modules:

- `exp/`: Module containing the explanation mechanisms
- `filesystem/`: Module for reading and writing files
- `search/`: Module for the internal data structure used to track points and search
- `util/`: Module with some useful code used throughout more modules
- `view/`: Module containing everyhing pertaining to the actual viewer and renderer
- `main.rs`: The command line definition.
- `generate.rs`: Code used for generating synthetic datasets like the cube

## Python

In the `python-repl` and `inverse_projection` both contains python sub projects, the `python-repl` can be used to quickly prototype some calculations or run a projection on an original dataset in a `Jupyter` environment. The `inverse_projection` contains the code from Matheus after I did some cleanup, it can be used to quickly gather the datasets and run the inverse projection technique. Both contain their own readme.

## Misc

the `notes` folder is currently filled with quick markdown files contains notes I made while working on the tool. The `data` folder will be used to store all the datasets. Finally the `kernels` folder, this contains cuda compute kernels that I used during my testing with `cuda` FFI. These are not relevant right now but might become so later on.
