#!/usr/bin/python

from abc import ABCMeta, abstractmethod


# Abstract class for Nucleo communication
class Communication(metaclass=ABCMeta):
    def __init__(self, manager):
        self.manager = manager

    @abstractmethod
    def read_data(self):
        pass

    @abstractmethod
    def send(self, grip: int):
        pass
