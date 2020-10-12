# Folder containing a Jupyterlab environment

This directory is used for some quick protyping and to call ScikitLearn functions. Running the `./run.sh` script will start a jupyter interactive python site that you can use for this. It is very useful when verifying math operations against numpy and to run  stuff like PCA / TSNE.

## Dependencies

Running the code in here will require you to run python 3.8 with `pipenv` installed. To use this you might need to install the `deadsnakes` ppa if are not on ubuntu 20.04. Instruction on can be found here: https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa.

Once you have the correct version of python installed you will need `pipenv`.
This will create a virtual python environment with the dependecies specified in `Pipfile`. Guide on how to install this can be found here: https://pipenv-fork.readthedocs.io/en/latest/install.html#installing-pipenv.

Once this is done you can let `pipenv` install the required dependencies for you, do this by running `pipenv sync`. After you can run `pipenv run jupyter lab --ip=0.0.0.0 --port=8080` or run `./run.sh` to start the jupyter repl in your browser of choise.
