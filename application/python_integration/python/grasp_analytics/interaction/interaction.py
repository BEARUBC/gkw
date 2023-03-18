#!/usr/bin/python

from abc import ABCMeta, abstractmethod
from threading import Thread


# Abstract class for user interaction
class Interaction(Thread, metaclass=ABCMeta):
    def __init__(self, thread_id, name, queue):
        Thread.__init__(self, daemon=True)
        self.threadID = thread_id
        self.name = name
        self.grip = ""
        self.queue = queue

    @abstractmethod
    def run(self):
        pass

    @abstractmethod
    def received_grip_callback(self, user_in):
        pass

    @staticmethod
    @abstractmethod
    def deactivate():
        pass

    @staticmethod
    @abstractmethod
    def reactivate():
        pass
