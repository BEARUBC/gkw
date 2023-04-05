import argparse
import numpy as np
import pandas as pd
from pathlib import Path

from src.grasp_analytics.definitions import ROOT_PATH, SETTINGS

parser = argparse.ArgumentParser(description="Generate FSR data")
parser.add_argument("out_path", type=str, help="Relative path to output directory")
parser.add_argument(
    "--mode",
    type=str,
    default="random",
    help="Read from a file with a specified path relative to the root directory",
)
parser.add_argument(
    "--size", type=int, default=1000, help="Number of frames of data to generate"
)

args = parser.parse_args()

dims = SETTINGS["fsr_matrix"]["dims"]


def generate_random_frame(raw=True) -> np.ndarray:
    raw_frame = np.random.randint(0, SETTINGS["fsr_matrix"]["resolution"], dims)
    if not raw:
        raw_frame = raw_frame / SETTINGS["fsr_matrix"]["resolution"]
    return raw_frame


generation_modes = {"random": generate_random_frame}

print("Generating", args.size, "frames using", args.mode)
generation_function = generation_modes.get(args.mode, lambda: "random")
generated_data = np.array(
    [np.ndarray.flatten(generation_function()) for _ in range(args.size)]
)
data_df = pd.DataFrame(generated_data)


# Rename columns
data_df.columns = [
    str(x // dims[0]) + "_" + str(x % dims[0]) for x in range(dims[0] * dims[1])
]


output_path = Path(args.out_path)
print("Saving data to", output_path)
data_df.to_csv(output_path)
