from kivy.app import App
from kivy.uix.label import Label
from kivy.uix.gridlayout import GridLayout
from src.gui.gui_button import GuiButton
from kivy.properties import StringProperty
from kivy.core.window import Window

knownGrips = ["mug", "pinch", "ball", "hammer", "flat", "test"]


class GraspGui(GridLayout):

    status = StringProperty()
    Window.clearcolor = (1, 1, 1, 1)

    # init the grid layout and generate buttons
    def __init__(self, **kwargs):
        super(GraspGui, self).__init__(**kwargs)
        self.cols = 2
        self.inside = GridLayout()
        self.inside.size_hint = 1.8, 1
        self.inside.cols = int(len(knownGrips) / 3)

        for i in range(len(knownGrips)):
            self.actionButton = GuiButton(label=knownGrips[i], grip=i)
            self.inside.add_widget(self.actionButton)
        self.add_widget(self.inside)

        self.submit = Label(text="status", font_size="72sp", color=(0, 0, 0, 1))
        self.add_widget(self.submit)


class GuiApp(App):
    def build(self):
        return GraspGui()


if __name__ == "__main__":
    GuiApp().run()
