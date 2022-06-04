#!/usr/bin/env python3
from subprocess import Popen, PIPE
from threading import Thread
from time import sleep
import uuid
import json

# Create child process and set up pipes for interactions
#
# any process that can be called in the command line can be substituted for the child process
# to do so, substitute the array parameter for the command used to run the child in the command line
#
# e.g. to use child_process.py (ran as "python child_process.py" in command line)
# pass ['python', 'child_process.py']
p = Popen(
    ['python', 'wrapper.py'], 
    stdin=PIPE, 
    stdout=PIPE, 
    stderr=PIPE, 
    text=True
)


# function is started as thread
def read_stdin():
    while(True):
        data_in = p.stdout.readline()
        if (data_in!=''):
            print('(Parent Read Thread) Received packet from child process:', data_in)
            
            response_packet = json.loads(data_in)
            print("(Parent Read Thread) Received data from child process:", response_packet['response'])


def main():
    # start read_stdin() as daemon thread so it is killed when program terminates
    read_thread = Thread(target=read_stdin, daemon=True)
    read_thread.start()

    while(True):
        job = input("What would you like to do: Capapitalize (C), Add Ten (T):, Haptic Feedback(H): ")

        if job == 'C':
            request = input("What would you like to capitalize? ")

            request_packet = {
                'request_id': str(uuid.uuid4()),
                'request_type': 'capitalize',
                'params': request
            }
            request_packet_str = json.dumps(request_packet) + '\n'

            p.stdin.write(request_packet_str)
            p.stdin.flush()
            print(f"(Parent Write Thread) Written {request_packet} to child")
            sleep(0.1)

        if job == 'T':
            num = input("Please input a number ")

            request_packet2 = {
                'request_id': str(uuid.uuid4()),
                'request_type': 'add_ten',
                'params': num
            }
            request_packet_str2 = json.dumps(request_packet2) + '\n'

            p.stdin.write(request_packet_str2)
            p.stdin.flush()
            print(f"(Parent Write Thread) Written {request_packet2} to child")
            sleep(0.1)

        if job == 'H':
            request3 = input("What would you like to Haptic Feedbackize? ")

            request_packet3 = {
                'request_id': str(uuid.uuid4()),
                'request_type': 'haptic_feedback',
                'params': request3
            }
            request_packet_str3 = json.dumps(request_packet3) + '\n'

            p.stdin.write(request_packet_str3)
            p.stdin.flush()
            print(f"(Parent Write Thread) Written {request_packet3} to child")
            sleep(0.1)



if __name__ == "__main__":
    main()

