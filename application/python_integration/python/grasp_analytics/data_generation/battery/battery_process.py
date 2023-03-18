class BatteryProcess:
    def __init__(self, processname: str, batteryusage: int, startstate: bool = False):
        self.processname: str = processname
        self.batteryusage: int = batteryusage
        self.turnedon: bool = startstate
