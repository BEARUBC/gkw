from .battery_process import BatteryProcess
from .battery_simulation import BatterySimulation


def main():
    emg_process = BatteryProcess("EMG", 5, startstate=True)
    hapticfeedback_process = BatteryProcess("Haptic_Feedback", 5, startstate=True)
    camera_process = BatteryProcess("Camera", 3, startstate=True)
    test_process = BatteryProcess("Test", 10)

    process_list = [emg_process, hapticfeedback_process, camera_process, test_process]
    sleep_time = 5

    battery_simulation = BatterySimulation(process_list, sleep_time)
    battery_simulation.run_simulation()


main()
