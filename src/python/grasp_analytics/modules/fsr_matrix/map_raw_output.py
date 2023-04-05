import sys

import pandas as pd
import numpy as np
from serial import Serial


def open_serial_connection():
    # COM ports are managed differently based on OS
    if sys.platform.startswith("linux"):
        return Serial("/dev/ttyACM0", 115200, timeout=1)
    elif sys.platform.startswith("win"):
        return Serial("COM5", 115200, timeout=1)
    else:
        raise EnvironmentError(sys.platform + " is an unsupported platform")


def collect_reading(ser):
    line = ser.readline().strip()
    reading = [
        int(x) for x in line.decode().split()
    ]  # convert reading to 1D numpy array
    if len(reading) != 160:
        return False
    return reading


def serial_fsr_collect(num_readings):
    ser = open_serial_connection()
    ser.flushInput()
    ser.readline()  # omit garbage row
    out_arr = []
    try:
        for i in range(
            num_readings
        ):  # read num_readings inputs from the FSR matrix and return them as 2d array
            reading = collect_reading(ser)
            if not reading:
                print("row", i, "failed")
                continue
            out_arr.append(reading)
            print(i, "out of", 10000)

    except ValueError:
        print(
            "The serial connection did not completely transfer the readings. Try replugging the serial connection."
        )

    return np.asarray(out_arr)


# this function is meant to be used by the tensorflow_pipeline2 file, should move eventually
def clean_list():
    data_list = serial_fsr_collect(20)
    pd_ob = pd.DataFrame(data_list)
    pd_ob.drop(pd_ob.columns[len(pd_ob.columns) - 1], axis=1, inplace=True)
    pd_ob = pd_ob.apply(pd.to_numeric)
    readable_list = pd_ob.values.tolist()

    # print(readable_list)
    return readable_list


if __name__ == "__main__":

    object_type = input(
        "Enter an object type (wcu: wooden cube, wc:wood cylinder, fs: foam sphere, n: nothing: "
    )
    data_one_list = serial_fsr_collect(10000)
    pd_ob = pd.DataFrame(data_one_list)
    # pd_ob.drop(pd_ob.columns[len(pd_ob.columns) - 1], axis=1, inplace=True)

    print(pd_ob)

    if input("save this data? (y/N)").lower() in ["yes", "y"]:
        if object_type == "wc":
            pd_ob["label"] = 3
            export_csv = pd_ob.to_csv(
                "export_dataframe_wood_cylinder.csv", index=None, header=True
            )  # Don't forget to add '.csv' at the end of the path

        if object_type == "wcu":
            pd_ob["label"] = 2

            export_csv = pd_ob.to_csv(
                "export_dataframe_wood_cube.csv", index=None, header=True
            )  # Don't forget to add '.csv' at the end of the path

        elif object_type == "fs":
            pd_ob["label"] = 1
            export_csv = pd_ob.to_csv(
                "export_dataframe_foam_sphere.csv", index=None, header=True
            )  # Don't forget to add '.csv' at the end of the path

        elif object_type == "n":
            pd_ob["label"] = 0
            export_csv = pd_ob.to_csv(
                "export_dataframe_nothing.csv", index=None, header=True
            )  # Don't forget to add '.csv' at the end of the path
        print("data saved")
