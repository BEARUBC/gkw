from roboclaw_3 import Roboclaw
from time import sleep

class Roboclaw_CTRL:
    def __init__(self):
        self.roboclaw = Roboclaw("/dev/ttyS0", 38400)
        self.roboclaw.Open()
        #.roboclaw.ForwardM1(0x80,63)	
        #self.roboclaw.BackwardM1(0x80,0)
        return

    def run(self, str_num):
		scale = int(str_num)
        self.roboclaw.ForwardM1(0x80,30)
        self.roboclaw.BackwardM1(0x80,0)
        return f'RECEIVED -> {str_num}'