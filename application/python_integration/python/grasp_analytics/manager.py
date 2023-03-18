#!/usr/bin/python

from .interaction.interaction_touchscreen import InteractionTouchscreen
from .interaction.interaction_voice import InteractionVoice
from .communication.communication_uart import CommunicationUART
import threading
from queue import Queue
from src.grasp_analytics.definitions import SETTINGS


class Manager:
    commTimeout = 300.0

    def __init__(self):
        self.settings = SETTINGS
        self.gripQueue = Queue()
        self.commEvent = threading.Event()
        self.ts = InteractionTouchscreen(1, "Touch Screen", self.gripQueue)
        self.voice = InteractionVoice(2, "Voice", self.gripQueue)
        self.comm = CommunicationUART(self)
        self.state = {}

    def set_state(self, state):
        self.state = state
        print("Set state of manager", state)

    def manage(self):
        self.ts.start()
        self.voice.start()
        try:
            while True:
                self.state["grip"] = self.gripQueue.get(block=True)
                if self.state["grip"].strip().lower() in SETTINGS["grips"]:
                    InteractionTouchscreen.deactivate()
                    InteractionVoice.deactivate()
                    self.comm.send(self.state)
                    InteractionTouchscreen.reactivate()
                    InteractionVoice.reactivate()

        except KeyboardInterrupt:
            print("Cancelled")
            # self.ts.join()
            self.voice.join()
