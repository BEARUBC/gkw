import random
import time
from typing import List

from .battery_process import BatteryProcess


class BatterySimulation:
    def __init__(self, processeslist: List[BatteryProcess], buffertime: float):
        self.processes = processeslist
        self.batterylife = 100
        self.buffertime = buffertime

    def reduce_battery(self):
        for process in self.processes:
            depletion = process.batteryusage
            is_turned_on = process.turnedon
            name = process.processname

            if is_turned_on:
                if self.batterylife - depletion < 0 and self.batterylife > 0:
                    self.batterylife = 0
                    print(
                        "Battery = "
                        + str(self.batterylife)
                        + " ("
                        + name
                        + " -"
                        + str(depletion)
                        + ")"
                    )
                elif self.batterylife - depletion < 0 and self.batterylife <= 0:
                    self.batterylife = 0
                else:
                    self.batterylife -= depletion
                    print(
                        "Battery = "
                        + str(self.batterylife)
                        + " ("
                        + name
                        + " -"
                        + str(depletion)
                        + ")"
                    )

    def run_simulation(self, change_frequency: float = 0.40):
        rand_cutoff = 100 * change_frequency

        while self.batterylife > 0:
            self.reduce_battery()

            rand_number = random.randint(0, 100)
            if rand_number <= rand_cutoff:
                selected_index = random.randint(0, len(self.processes) - 1)
                selected_process = self.processes[selected_index]

                if selected_process.turnedon:
                    selected_process.turnedon = False
                else:
                    selected_process.turnedon = True
                print(
                    "Set "
                    + str(selected_process.processname)
                    + " to "
                    + str(selected_process.turnedon)
                )

            time.sleep(self.buffertime)

        print("Battery Depleted")
