import os
from pathlib import Path

from kivy.uix.image import Image
from kivy.uix.behaviors import ButtonBehavior
from kivy.atlas import Atlas

import sys

sys.path.append("")

from src.grasp_analytics.manager import Manager

grip = ["mug", "pinch", "ball", "hammer", "flat", "test"]
gripPress = [
    "mug_down",
    "pinch_down",
    "ball_down",
    "hammer_down",
    "flat_down",
    "test_down",
]


class GuiButton(ButtonBehavior, Image):
    def __init__(self, *args, **kwargs):
        super(GuiButton, self).__init__()
        self.working_dir = Path(os.path.dirname(os.path.abspath(__file__)))
        self.atlas = Atlas(
            str(Path(self.working_dir) / "components/images/guiatlas.atlas")
        )

        self.argument = kwargs.get("grip", 404)
        self.label = kwargs.get("label", None)

        self.texture = self.atlas[grip[self.argument]]

    def on_press(self):
        manage = Manager()
        self.texture = self.atlas[gripPress[self.argument]]
        manage.set_state(grip[self.argument])
        print(grip[self.argument])

    def on_release(self):
        self.texture = self.atlas[grip[self.argument]]
