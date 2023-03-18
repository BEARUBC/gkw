"""
This script currently evaluates the performance of the crop CNN model
"""


import pickle

import numpy as np

from src.grasp_analytics.definitions import ROOT_PATH, SETTINGS
from .crop_cnn import CropCNNSelector

data_path = (
    ROOT_PATH
    / SETTINGS["grip_select"]["results_dir"]
    / "20210914_235655_train_size_25.pickle"
)

with open(str(data_path), "rb") as data_file:
    data = pickle.load(data_file)

images = [np.asarray(x[0]) for x in data]
labels = [x[3] for x in data]

selector = CropCNNSelector()
selector_evaluation = selector.evaluate(images, labels)

# Print the returned confusion matrix of our selector
print(selector_evaluation)
