from pathlib import Path
from typing import Optional

import pandas as pd
import numpy as np

from .reader import DataReader


class FileReader(DataReader):
    def __init__(self, file_path: Path):
        super().__init__()
        self.df = pd.read_csv(file_path)
        self.counter = 0

    def get_frame(self, raw=False) -> Optional[np.ndarray]:
        if not self.available:
            return None

        frame_row = self.df.iloc[[self.counter]]
        self.counter += 1
        if self.counter >= len(self.df.index):
            self.available = False
        reading = np.reshape(frame_row.to_numpy()[:, 1:], tuple(self.settings["dims"]))

        if not raw:  # Normalize reading
            reading = reading / self.settings["resolution"]
        return reading
