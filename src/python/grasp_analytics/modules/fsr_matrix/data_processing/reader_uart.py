import sys
from typing import Optional

from serial import Serial
import numpy as np

from src.grasp_analytics.definitions import SETTINGS
from .reader import DataReader


class UartReader(DataReader):
    def __init__(self, port=None):
        super().__init__()
        self.settings.update(SETTINGS["communication"]["UART"])
        self.ser = self.open_serial_connection()
        self.ser.flushInput()
        self.ser.readline()  # omit garbage row

    def open_serial_connection(self, port=None):
        if port is not None:
            return Serial(port, self.settings["baud_rate"], timeout=1)
        # COM ports are managed differently based on OS
        elif sys.platform.startswith("linux"):
            return Serial(
                self.settings["unix_port"], self.settings["baud_rate"], timeout=1
            )
        elif sys.platform.startswith("win"):
            return Serial(
                self.settings["win_port"], self.settings["baud_rate"], timeout=1
            )
        else:
            self.available = False
            raise EnvironmentError(sys.platform + " is an unsupported platform")

    def read_line(self):
        line = self.ser.readline().strip()
        reading = [
            int(x) for x in line.decode().split()
        ]  # convert reading to 1D numpy array
        if len(reading) != self.reading_length:
            return False
        return reading

    def get_frame(self, raw=False) -> Optional[np.ndarray]:
        line = self.read_line()
        reading = np.reshape(line, tuple(self.settings["dims"]))

        if not raw:  # Normalize reading
            reading = reading / self.settings["resolution"]
        return reading
