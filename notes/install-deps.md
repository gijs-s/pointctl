# Dependecies needed to run this program.

## Rust

Rust can be installed with one simple command:

``` sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you do not feel comfortable piping a website into shell I completely understand, read up about how to do manual installs here: https://www.rust-lang.org/tools/install

For the rendering library you will also need `libxcb-xinput-dev` and `libxcb1-dev` if you are running on linux. For the math section I use `lapack` which relies on `openblas`, this requires `gfortran` and `libgfortran-{7,8,9}-dev` to be compiled in the background.

## Cuda

For installing CUDA I highly recommend using the prepacked version provided by the people over at System76. With this you are just one command away from installing cuda. To add the repository to apt use the following commands. More info on system76 can be found here: https://support.system76.com/articles/cuda/.

``` sh
sudo echo "deb http://apt.pop-os.org/proprietary bionic main" | sudo tee -a /etc/apt/sources.list.d/pop-proprietary.list
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-key 204DD8AEC33A7AFF
sudo apt update
```

Once the repository is added you can install cuda and cudnn, I recommend getting the latests but it is also nice to have 10.0 installed for tools like tensorflow.

``` sh
sudo apt install system76-cuda-10.0 system76-cuda-10.2 system76-cudnn-10.0 system76-cudnn-10.2
```

Sadly cuda does not work with the latest gcc compilers, to use it you first need to switch to gcc 7.

``` sh
# install the latests few gcc compilers
sudo apt-get install gcc-7 g++-7 gcc-8 g++-8
```

You might have noticed that now you have multiple versions of the same libraries installed, to easily switch between these you can use `update-alternatives`. This allows for easy switching of system wide cuda / gcc versions. To make use of this you can input the following commands.

``` sh
# Add the alternatives for handling multiple versions being installed
sudo update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-9 90 --slave /usr/bin/g++ g++ /usr/bin/g++-9
sudo update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-8 80 --slave /usr/bin/g++ g++ /usr/bin/g++-8
sudo update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-7 70 --slave /usr/bin/g++ g++ /usr/bin/g++-7

# Add simple switch alias to bashrc so you can forget the full commands and reload the shell
echo "alias pick='sudo update-alternatives --config'" >> ~/.bashrc
. ~/.bashrc

# Use the new command to cuda or gcc switch versions. (cuda 10.2 works with gcc 8. 10.0 with 7)
pick cuda
pick gcc
```

## Python

My program also uses a python 3.{7,8} and `pipenv` for the python packages. To use this you might need to install the `deadsnakes` ppa if you are not on Ubuntu 20.04. Instruction on can be found here: https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa. After you can install `pipenv` according to this guide: https://pipenv-fork.readthedocs.io/en/latest/install.html#installing-pipenv.

When this installed running `pipenv sync` in the `jupyter/` will create a virtual python environment that contains all the dependencies. This directory is currently used for some quick protyping and to call ScikitLearn functions. Running the `./run.sh` script will start a jupyter interactive python site that you can use for this.

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