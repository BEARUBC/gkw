from enum import IntEnum

from src.grasp_analytics.module import Module
from crop_cnn.crop_cnn import CropCNNSelector
from .selector import GripSelector
from .mobilenet.analyzer import MobileNetAnalyzer


class GripSelectModel(IntEnum):
    CROP_CNN = (0,)
    MOBILENET = 1


class GripSelect(Module):
    def __init__(self, mode: GripSelectModel):
        self.mode = mode
        if self.mode == GripSelectModel.CROP_CNN:
            self._selector: GripSelector = CropCNNSelector()
        else:
            self._selector: GripSelector = MobileNetAnalyzer()

    def run(self, input_json: dict) -> dict:
        pass
