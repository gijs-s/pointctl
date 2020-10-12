# Python code for gathering datasets

This directory contains the code used for the inverse projection works from Matheus. Here I also included the a neat download script to gather all the datasets.

## Dependencies

Running the code in here will require you to run python 3.7 with `pipenv` installed. To use this you might need to install the `deadsnakes` ppa if are on the bleeding edge with Ubuntu 20.04. Instruction on can be found here: https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa.

Once you have the correct version of python installed you will need `pipenv`.
This will create a virtual python environment with the dependencies specified in `Pipfile`. Guide on how to install this can be found here: https://pipenv-fork.readthedocs.io/en/latest/install.html#installing-pipenv.

Once this is done you can let `pipenv` install the required dependencies for you, do this by running `pipenv sync`. After you can run `pipenv run ./get_datasets.py` to download all the datasets to `../data`!

## The neural networking

The neural networking of course preforms best when using CUDA and not only CPU, for more information on installing this I'd like to refer to `notes/install-deps.md`. This is currently not connected to the rust framework. Due to the simple nature of the network and the training process I might re-implement the entire thing using pytorch with rust bindings.