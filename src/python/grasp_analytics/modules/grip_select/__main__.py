import cv2

from src.definitions import ROOT_PATH, SETTINGS
from .grip_select import GripSelect, GripSelectModel

img_path = ROOT_PATH / SETTINGS["grip_select"]["data_dir"] / "images/cup/cup_001.jpg"
im = cv2.imread(str(img_path))
grip_select = GripSelect(GripSelectModel.CROP_CNN)
classification = grip_select._selector.classify_image(im)
