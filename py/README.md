# Rust-Python Communications
This directory contains:
modules/
    .py files
io/
    input/
        emg_input.json
        fsr_input.json
        camera.json
    output/
        emg_output.json
config.json


These files are used in communications with the Python analytics.
They should not be tracked with VC because they should be used as buffers only.


The io folder and each respective folder within it represents input/output. 
The input folder will contain json files that are used to send information to
the analytics team modules. The output folder will contain json files that are used
to receive request data from the analytics team.

The modules folder will contain the python files written by the analytics team.

The config.json file will contain configuration details specified by the analytics team.