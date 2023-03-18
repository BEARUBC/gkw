from typing import Optional

import numpy as np
from abc import abstractmethod, ABCMeta

from src.grasp_analytics.definitions import SETTINGS


class DataReader(metaclass=ABCMeta):
    def __init__(self):
        self.settings = SETTINGS["fsr_matrix"]
        self.reading_length = self.settings["dims"][0] * self.settings["dims"][1]
        self.available = True

    @abstractmethod
    def get_frame(self, raw=False) -> Optional[np.ndarray]:
        pass
