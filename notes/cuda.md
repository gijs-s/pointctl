# CUDA installation

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