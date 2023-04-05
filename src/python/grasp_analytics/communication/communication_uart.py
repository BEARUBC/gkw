#!/usr/bin/python
import json
import time

from src.grasp_analytics.definitions import SETTINGS
from .communication import Communication
import serial


class CommunicationUART(Communication):
    ser = serial.Serial()

    # We don't require a manager in case we want to run as a standalone
    def __init__(self, manager=None):
        self.settings = SETTINGS["communication"]["UART"]
        self.manager = manager
        if self.manager is not None:
            super().__init__(self.manager)
        # Configure serial port
        self.ser.port = self.settings["port"]
        self.ser.baudrate = self.settings["baud_rate"]
        self.ser.parity = serial.PARITY_NONE
        self.ser.stopbits = serial.STOPBITS_ONE
        self.ser.bytesize = serial.EIGHTBITS
        self.ser.timeout = 1
        # Open serial port
        self.ser.open()

    # Reads data from nucleo
    def read_data(self):
        try:
            data = self.ser.readline().decode()
        except Exception as e:
            print("Serial read failed:", e)
            return
        try:
            if self.manager is not None:
                self.manager.state = json.loads(data)
            else:
                data = data.replace("'", '"')
                print("Data:", json.loads(data))
        except Exception as e:
            print("Invalid data:", data, "error:", str(e))

    # Send encoded grip to nucleo
    def send(self, data: dict):
        encoded = (str(data) + "\n").encode()
        self.ser.write(encoded)


def main():
    com = CommunicationUART()
    while True:
        com.read_data()
        com.send({"test": "ok", "asd": 45})
        time.sleep(0.1)


if __name__ == "__main__":
    main()
