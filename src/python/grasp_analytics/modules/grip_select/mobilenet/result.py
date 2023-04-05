from dataclasses import dataclass
from typing import List

from src.grasp_analytics.utils import BoundingBox


@dataclass
class MobileNetResult:
    boxes: List[BoundingBox]
