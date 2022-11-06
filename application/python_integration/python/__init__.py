'''
This package contains the wrapper code for running UBC Bionics' analytics team's code.


EXPLANATION OF FILES
wrapper.py: wrapper code that bridges the parrent process and the analytics code. Instantiates all the analytics classes.
test.py: testing code for wrapper.py. test.py starts wrapper.py as a child process and sends requests through os pipes.
add_ten.py: contains a class used by wrapper.py for testing
capitalize.py: contains a class used by wrapper.py for testing


NOTES
To run haptic feedback from the command line call wrapper then use
{"request_id": "2", "request_type": "haptic_feedback", "params": "{\"fsr_strengths\":[1,2,3,4]}"}

To test wrapper.py, run it, and enter into stdin:
{"request_id": "2", "request_type": "capitalize", "params": "hi"}

To run haptic feedback from tests use
{"fsr_strengths":[1,2,3,4]}  

Note from Zach
For meeting mention changes to
module.py: changed run function, was originally pass, commented out another function
haptic feedback: renamed run to _process, changed import statement

Changes we need to ask James for:
- Change grasp-py to grasp_py

'''

