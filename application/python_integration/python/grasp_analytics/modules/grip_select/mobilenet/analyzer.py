from collections import namedtuple

import cv2
from typing import Optional

from src.grasp_analytics.definitions import ROOT_PATH
from .objects import OBJECT_GRIP_MAP
from ..selector import GripSelector
from src.grasp_analytics.utils import BoundingBox

MobileNetAnalysisResult = namedtuple(
    "MobileNetAnalysisResult", "class_name confidence bbox grip_type"
)


class MobileNetAnalyzer(GripSelector):
    def classify_image(self, image):
        pass

    def __init__(
        self,
        confidence_threshold: float = 0.65,
        classes_path: str = None,
        config_path: str = None,
        weights_path: str = None,
    ):
        self._path = ROOT_PATH / "grip_select/mobilenet/"

        self._classes_path = (
            classes_path if classes_path else str(self._path / "coco.names")
        )
        self._config_path = (
            config_path
            if config_path
            else str(self._path / "ssd_mobilenet_v3_large_coco_2020_01_14.pbtxt")
        )
        self._weights_path = (
            weights_path
            if weights_path
            else str(self._path / "frozen_inference_graph.pb")
        )
        self.confidence_threshold = confidence_threshold

        with open(str(self._classes_path), "rt") as f:
            self._class_names = f.read().rstrip("\n").split("\n")

        self._model = self._create_model()

    def _create_model(self):
        net = cv2.dnn_DetectionModel(self._weights_path, self._config_path)
        net.setInputSize(320, 320)
        net.setInputScale(1.0 / 127.5)
        net.setInputMean((127.5, 127.5, 127.5))
        net.setInputSwapRB(True)
        return net

    def analyze_image(self, image) -> Optional[MobileNetAnalysisResult]:
        height, width, channels = image.shape
        top_left = (240, 160)
        bottom_right = (400, 320)

        image_dims = (width, height)
        image_bbox = BoundingBox(0, 0, *image_dims)
        cv2.rectangle(image, top_left, bottom_right, (0, 0, 255), thickness=1)

        classIds, confs, bbox = self._model.detect(image, confThreshold=0.5)
        bbox = [BoundingBox(*b) for b in bbox]
        dists = [b.center.sqr_dist_to(image_bbox.center) for b in bbox]

        if len(classIds) == 0:
            return None

        box_data = list(zip(classIds.flatten(), confs.flatten(), bbox, dists))
        box_data = [
            x for x in box_data if self._class_names[x[0] - 1] in OBJECT_GRIP_MAP
        ]
        if len(box_data) == 0:
            return None

        # Take min based on dist to center
        selected_box_data = min(box_data, key=lambda box: box[3])
        selected_box: BoundingBox = selected_box_data[2]
        selected_class_id = selected_box_data[0] - 1

        if not selected_box:
            return None

        confidence = selected_box_data[1]
        if confidence < self.confidence_threshold:
            return None

        class_name = self._class_names[selected_class_id]
        return MobileNetAnalysisResult(
            class_name, confidence, selected_box_data[2], OBJECT_GRIP_MAP[class_name]
        )
