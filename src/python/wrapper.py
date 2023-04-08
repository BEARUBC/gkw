from threading import Thread, Lock
import queue
from time import sleep
import json
import sys
import os
from capitalize import Capitalize
from add_ten import Add_Ten
from roboclaw_example import Roboclaw_CTRL
# from grasp_analytics.modules.emg import emg

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

def init():
    init_param_raw = sys.stdin.readline()
    init_param = json.loads(init_param_raw)

    path_to_analytics = init_param['path_to_analytics']

    sys.path.append(path_to_analytics + 'python/grasp_analytics/modules/emg')
    from emg import EMG
    m_emg=EMG()

    return m_emg


def main():
    m_emg=init()
    capitalize=Capitalize()
    add_ten=Add_Ten()
    motor_test = Roboclaw_CTRL()
    # print(m_emg)
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

