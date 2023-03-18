from threading import Thread, Lock
import queue
from time import sleep
import json
import sys
from capitalize import Capitalize
from add_ten import Add_Ten
from grasp_analytics.modules.emg import emg
# from grasp_py.src.grasp_analytics.module import Module
# from grasp_py.src.grasp_analytics.modules.haptic_feedback.haptic_module import HapticFeedback


q=queue.Queue()
qLock=Lock()

def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)

def checkStdin():
    while True:
        request = sys.stdin.readline()
        if (request):
            qLock.acquire(blocking=1)
            q.put(request)
            qLock.release()

def main():
    capitalize=Capitalize()
    add_ten=Add_Ten()
    m_emg=emg.EMG()
    # haptic_feedback=HapticFeedback()

    r_Thread = Thread(target=checkStdin, daemon=True)
    r_Thread.start()

    while True:
        qLock.acquire(blocking=1)
        if(q.qsize()==0):
            qLock.release()
            sleep(0.1)
        else:
            J_obj=q.get()
            process=json.loads(J_obj)
            eprint("AAAAAA ", process["params"], " AAAAAA", flush=True)
            response_packet = {
                'request_id': process['request_id'],
                'valid_bit': 1,
                'data': locals()[process["request_type"]].run(process["params"])
            }
            sys.stdout.write(json.dumps(response_packet) + "\n")
            sys.stdout.flush()
            qLock.release()

if __name__ == "__main__":
    main()

