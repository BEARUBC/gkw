import matplotlib.pyplot as plt
import numpy as np
import math as m

from src.grasp_analytics.definitions import SETTINGS


class RandomWalkGenerator:
    def __init__(self):
        self.settings = SETTINGS["haptic_feedback"]["randomwalk_generator"]
        self.plotsize = self.settings["default_plotsize"]
        self.datapoints = self.settings["default_numdatapoints"]
        self.randomwalk = []

    def generator(self):
        datapoints = self.datapoints

        steps = np.random.standard_normal(datapoints)
        steps[0] = 0
        randomwalk = np.cumsum(steps)

        for i in range(len(randomwalk)):
            if randomwalk[i] < 0:
                randomwalk[i] *= -1

        self.randomwalk = randomwalk

    def plotter(self):
        plot_size = self.plotsize
        rw = self.randomwalk

        plt.figure(figsize=plot_size)
        plt.plot(rw)
        plt.title("Simulated Random Walk")
        plt.show()

    def rw_setter(self, new_randomwalk: list):
        self.randomwalk = new_randomwalk
