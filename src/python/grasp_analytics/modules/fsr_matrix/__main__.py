import argparse
from pathlib import Path

from src.definitions import ROOT_PATH
from .classifier.matrix_classifier import MatrixClassifier
from .data_processing.reader_file import FileReader
from .data_processing.reader_uart import UartReader
from .visualizer import MatrixVisualizer

parser = argparse.ArgumentParser(description="Visualize FSR data in real time")
parser.add_argument(
    "--file",
    type=str,
    default=None,
    help="Read from a file with a specified path relative to the root directory",
)
parser.add_argument(
    "--file_absolute_path",
    type=str,
    default=None,
    help="Read from a file with a specified absolute path",
)
parser.add_argument(
    "--port",
    type=str,
    default=None,
    help="Read from a serial connection with a specified port",
)
parser.add_argument(
    "--classify", type=bool, default=False, help="Display shape classification"
)

args = parser.parse_args()

if args.port is not None:
    _reader = UartReader(port=args.port)
elif args.file_absolute_path is not None:
    _reader = FileReader(Path(args.file_absolute_path))
elif args.file is not None:
    _reader = FileReader(ROOT_PATH / args.file)
else:
    raise Exception("No input method specified")

visualizer = MatrixVisualizer()
if args.classify:
    _classifier = MatrixClassifier()
    visualizer.start(_reader, _classifier)
else:
    visualizer.start(_reader)
