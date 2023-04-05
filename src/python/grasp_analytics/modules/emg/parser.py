import argparse
from typing import Optional

import pandas
import pathlib
import numpy as np
import pandas as pd
import plotly.express as px
import plotly.io as pio

from src.grasp_analytics.definitions import ROOT_PATH, SETTINGS

pio.renderers.default = "browser"


def normalize_data(data: pd.Series) -> pd.Series:
    return np.square(data)


class EMGParser:
    def __init__(self, file_path: pathlib.Path):
        self.settings = SETTINGS["emg"]
        self.available = True
        self.df = pd.read_csv(file_path)
        self.counter = 0

    def _read(self):
        if not self.available:
            return None
        frame_row = self.df.iloc[[self.counter]]
        self.counter += 1
        if self.counter >= len(self.df.index):
            self.available = False
        return frame_row

    def get_reading(self, raw=False):
        if not self.available:
            return None

        reading = self._read()
        if reading is None:
            raise Exception("Read from EMG data failed.")

        if not raw:  # Square and normalize reading
            reading = normalize_data(reading)
        return reading

    def get_all(self, raw=False):
        if not raw:
            return normalize_data(self.df)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Parse EMG Data")
    parser.add_argument("file", type=str, help="Read from a file with a specified path")
    args = parser.parse_args()

    emg_parser = EMGParser(pathlib.Path(args.file))
    data_df = emg_parser.get_all()
    electrode_fig_1 = px.line(
        data_df, y=data_df.columns[0], title="Electrode 1 Graph (index)"
    )
    electrode_fig_1.show()

if __name__ == "visualizer":
    parser = argparse.ArgumentParser(description="Parse EMG Data")
    parser.add_argument("file", type=str, help="Read from a file with a specified path")
    args = parser.parse_args()

    emg_parser = EMGParser(pathlib.Path(args.file))
    data_df = emg_parser.get_all()
# index_df.columns = ["electrode_" + str(i + 1) for i in range(len(index_df.columns))]
#
# for col in index_df.columns:
#     index_df[col] = index_df[col] ** 2
#
# electrode_fig_1 = px.line(index_df, y=index_df.columns[0], title="Electrode 1 Graph (index)")
# # electrode_fig_2 = px.line(index_df, y=index_df.columns[0], title="Electrode 2 Graph (index)")
# ##electrode_fig_3 = px.line(index_df, y=index_df.columns[0], title="Electrode 3 Graph (index)")
#
# electrode_fig_1.show()
