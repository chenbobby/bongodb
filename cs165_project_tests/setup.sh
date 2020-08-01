#!/bin/bash

####
# This script has only been tested on a Debian 10 host machine.
#
# Run this script from the "cs165_project_test" directory to setup your Python
# virtual environment for data generation and CS165 tests.
####


# Install Python 3 and create a virtual env in the "cs165_project_tests" directory.
sudo apt install python3 python3-venv
python3 -m venv ./venv

# Install Python dependencies in the virtual env.
source ./venv/bin/activate
pip install -r requirements.txt
deactivate

echo "Completed setup.sh"
