import torch
import torch.nn as nn
import torch.functional as F
import torchvision
from torch.utils.data import Dataset, DataLoader
from torchvision.transforms import ToTensor

from src.grasp_analytics.definitions import ROOT_PATH


class ConvBlock(nn.Module):
    def __init__(self, in_channels, out_channels, kernel_size, stride, padding):
        super(ConvBlock, self).__init__()
        self.conv1 = nn.Sequential(
            nn.Conv2d(in_channels=in_channels, out_channels=out_channels, kernel_size=kernel_size, stride=stride,
                      padding=padding),
            nn.ReLU()
        )

    def forward(self, X):
        x = self.conv1(X)
        return x


class AlexNet(nn.Module):
    def __init__(self, num_classes):
        super(AlexNet, self).__init__()
        self.convs = nn.Sequential(
            ConvBlock(in_channels=3, out_channels=7, kernel_size=3, stride=1, padding=0),
            nn.MaxPool2d(kernel_size=3, stride=2, padding=0),

            ConvBlock(in_channels=7, out_channels=14, kernel_size=5, stride=1, padding=2),
            nn.MaxPool2d(kernel_size=3, stride=2, padding=0),

            ConvBlock(in_channels=14, out_channels=28, kernel_size=7, stride=1, padding=1),
            ConvBlock(in_channels=28, out_channels=14, kernel_size=5, stride=1, padding=1),
            ConvBlock(in_channels=14, out_channels=7, kernel_size=3, stride=1, padding=1),

            nn.MaxPool2d(kernel_size=3, stride=2, padding=0)
        )
        self.linear = nn.Sequential(
            nn.Linear(in_features=28875, out_features=4096),
            nn.ReLU(),
            nn.Dropout(0.5),

            nn.Linear(in_features=4096, out_features=4096),
            nn.ReLU(),
            nn.Dropout(0.5),

            nn.Linear(in_features=4096, out_features=num_classes),
            nn.LogSoftmax(dim=-1)
        )

    def forward(self, X):
        X = self.convs(X)
        X = X.view(X.shape[0], -1)
        X = self.linear(X)
        return X


if __name__ == "__main__":
    path = "../../data/img_dir"
    data_path = ROOT_PATH / path
    data = torchvision.datasets.ImageFolder(str(data_path), transform=ToTensor())
    data_loader = DataLoader(data, batch_size=1)

    Neural_Net = AlexNet(5)
    for img, target in data_loader:
        print(img, target)
        y_pred = Neural_Net(img)
        print(y_pred)