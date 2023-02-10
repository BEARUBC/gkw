from PyQt5 import QtGui
from PyQt5.QtWidgets import QApplication, QMainWindow
from PyQt5.QtGui import QPainter, QBrush, QPen
from PyQt5.QtCore import Qt
import sys

cell_width = 50
cell_height = 50

num_rows = 6
num_cols = 6

class Window(QMainWindow):
    def __init__(self):
        super().__init__()

        self.title = "PyQt5 Drawing Rectangle"
        self.top = 100
        self.left = 100
        self.width = 680
        self.height = 500

        self.InitWindow()


    def InitWindow(self):
        self.setWindowIcon(QtGui.QIcon("icon.png"))
        self.setWindowTitle(self.title)
        self.setGeometry(self.top, self.left, self.width, self.height)


        self.show()


    def paintEvent(self, e):
    #     painter = QPainter(self)
    #     painter.setPen(QPen(Qt.black, 5, Qt.SolidLine))
    #     #painter.setBrush(QBrush(Qt.red, Qt.SolidPattern))
    #     painter.setBrush(QBrush(Qt.green, Qt.DiagCrossPattern))
    #
    #     painter.drawRect(100, 15, 400,200)
        painter = QPainter(self)
        painter.setPen(QPen(Qt.black, 5, Qt.SolidLine))
        # painter.setBrush(QBrush(Qt.red, Qt.SolidPattern))
        painter.setBrush(QBrush(Qt.green, Qt.DiagCrossPattern))

        for row in range(num_rows):
            for col in range(num_cols):
                painter.drawRect(row * cell_width, col * cell_height, cell_width, cell_height)


App = QApplication(sys.argv)
window = Window()
sys.exit(App.exec())