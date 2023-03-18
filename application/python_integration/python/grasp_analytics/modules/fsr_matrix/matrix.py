from src.grasp_analytics.definitions import TORCH_DEVICE, SETTINGS, ROOT_PATH
from .classifier.matrix_classifier import MatrixClassifier
from .data_processing.reader_file import FileReader
from .data_processing.reader_uart import UartReader


class FSRMatrix:
    def __init__(self):
        self.settings = SETTINGS["fsr_matrix"]
        if self.settings["reader"]["mode"] == "file":
            self.reader = FileReader(ROOT_PATH / self.settings["reader"]["file_path"])
        else:
            self.reader = UartReader()
        self.classifier = MatrixClassifier().to(TORCH_DEVICE)

    def get_current_obj(self):
        if self.reader.available:
            frame = self.reader.get_frame()
        return self.classifier.classify(frame)
