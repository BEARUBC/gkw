import numpy as np

from src.grasp_analytics.definitions import SETTINGS


class PeakDetector:
    def __init__(
        self, lag: int, threshold: float, influence: float, numthresholds: int
    ):
        self.settings = SETTINGS["emg"]["peak_detection"]
        self.lag: int = lag
        self.length = self.lag
        self.threshold: float = threshold
        self.influence: float = influence
        self.y: list = [0] * self.lag
        self.signals: list = [0] * self.lag
        self.filteredY = [0] * self.lag
        self.avgFilter: list = [0] * self.lag
        self.stdFilter: list = [0] * self.lag
        self.avgFilter[self.lag - 1] = self.settings["default_mean"]
        self.stdFilter[self.lag - 1] = self.settings["default_std"]
        self.data_points_seen = 0
        self.numthresholds = numthresholds

    def threshold_new_val(self, new_value):
        self.data_points_seen += 1
        self.y.append(new_value)
        self.y.pop(0)
        if self.data_points_seen < self.lag:
            return 0

        self.signals += [0]
        self.signals.pop(0)
        self.filteredY += [0]
        self.filteredY.pop(0)
        self.avgFilter += [0]
        self.avgFilter.pop(0)
        self.stdFilter += [0]
        self.stdFilter.pop(0)

        x = 1
        while x <= self.numthresholds:
            newthreshold = self.threshold * (x / self.numthresholds)
            if abs(self.y[-1] - self.avgFilter[-2]) > newthreshold * self.stdFilter[-2]:
                self.signals[-1] = x / self.numthresholds
                self.filteredY[-1] = (
                    self.influence * self.y[-1]
                    + (1 - self.influence) * self.filteredY[-2]
                )
                self.avgFilter[-1] = np.mean(self.filteredY[:-1])
                self.stdFilter[-1] = np.std(self.filteredY[:-1])
            elif x == 1:
                self.signals[-1] = 0
                self.filteredY[-1] = self.y[-1]
                self.avgFilter[-1] = np.mean(self.filteredY[:-1])
                self.stdFilter[-1] = np.std(self.filteredY[:-1])
            else:
                self.filteredY[-1] = self.y[-1]
                self.avgFilter[-1] = np.mean(self.filteredY[:-1])
                self.stdFilter[-1] = np.std(self.filteredY[:-1])
            x = x + 1

        return self.signals[-1]
