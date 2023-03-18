#!/usr/bin/python
from .interaction import Interaction
from src.grasp_analytics.definitions import SETTINGS


class InteractionTouchscreen(Interaction):
    activated = True

    def __init__(self, thread_id, name, queue):
        self.settings = SETTINGS["interaction"]["touchscreen"]
        super().__init__(thread_id, name, queue)
        self.grip_message = None

    @staticmethod
    def reactivate():
        InteractionTouchscreen.activated = True

    def received_grip_callback(self, user_in):
        if InteractionTouchscreen.activated:
            self.grip_message.value = user_in
            self.queue.put(user_in)
        else:
            print("blocked")

    def run(self):
        try:
            print("start Ts")
            app = App(title="Hello world")
            self.grip_message = Text(app, text="No Grip Selected.")
            grip_1 = PushButton(
                app, command=self.received_grip_callback, args=["Cup"], text="Cup"
            )
            app.display()
        except KeyboardInterrupt:
            pass

    @staticmethod
    def deactivate():
        InteractionTouchscreen.activated = False
