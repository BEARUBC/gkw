from src.grasp_analytics.definitions import TORCH_DEVICE
from .matrix_classifier import MatrixClassifier

matrix_classifier = MatrixClassifier().to(TORCH_DEVICE)
