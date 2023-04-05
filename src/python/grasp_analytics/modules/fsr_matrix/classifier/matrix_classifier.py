from abc import ABCMeta
from enum import Enum, auto

import numpy as np

import torch
import torch.nn as nn
import torch.nn.functional as F

from src.grasp_analytics.definitions import SETTINGS, TORCH_DEVICE


class ObjectShape(Enum):
    NOTHING = auto()
    FOAM_SPHERE = auto()
    WOOD_CUBE = auto()
    WOOD_CYLINDER = auto()


class MatrixClassifier(nn.Module, metaclass=ABCMeta):
    def __init__(self):
        super(MatrixClassifier, self).__init__()
        self.settings = SETTINGS["fsr_matrix"]
        self.model_settings = self.settings["classifier"]
        self.input_shape = tuple(self.settings["dims"])
        self.input_channels = self.model_settings["input_channels"]
        self.output_size = self.model_settings["output_size"]
        self._output_map = {
            0: ObjectShape.NOTHING,
            1: ObjectShape.FOAM_SPHERE,
            2: ObjectShape.WOOD_CUBE,
            3: ObjectShape.WOOD_CYLINDER,
        }

        # Model Layers
        self.conv1 = nn.Conv2d(
            in_channels=self.input_channels,
            out_channels=self.input_channels,
            kernel_size=3,
        )
        self.mp1 = nn.MaxPool2d(kernel_size=3)
        self.conv2 = nn.Conv2d(
            self.input_channels, self.input_channels * 2, kernel_size=3
        )
        self.mp2 = nn.MaxPool2d(kernel_size=2)
        self.conv_drop = nn.Dropout2d()
        self.fc1 = nn.Linear(77, 50)
        self.fc2 = nn.Linear(50, self.output_size)

    def forward(self, x: torch.tensor):
        """
        :param x: 2d torch tensor of inputs of shape (self.input_shape[0], self.input_shape[1])
        """
        if x.size() != self.input_shape:
            raise Exception(
                "Input tensor shape "
                + str(x.size())
                + "does not match expected size "
                + str(self.input_shape)
            )

        x = x.view(1, self.input_channels, *self.input_shape)

        in_size = x.size(0)
        x = self.conv1(x)
        x = F.relu(self.mp1(x))
        x = self.conv2(x)
        x = F.relu(self.mp2(x))
        x = F.relu(self.conv_drop(x))
        x = x.view(in_size, -1)
        x = F.relu(self.fc1(x))
        x = F.dropout(x, training=self.training)
        x = self.fc2(x)
        x = F.log_softmax(x, dim=1)
        return x

    def classify(self, fsr_frame: np.ndarray) -> ObjectShape:
        x = torch.from_numpy(fsr_frame).to(TORCH_DEVICE)
        out = self.forward(x)
        return self._output_map[out.argmax().item()]
