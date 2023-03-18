from .interaction import Interaction
from src.grasp_analytics.definitions import SETTINGS
import speech_recognition as sr


class InteractionVoice(Interaction):
    activated = True
    voice_grips = SETTINGS["grips"]
    voice_grips.append("test")
    known_grips = zip(voice_grips, [1.0 for _ in voice_grips])

    def __init__(self, thread_id, name, queue):
        self.settings = SETTINGS["interaction"]["voice"]
        super().__init__(thread_id, name, queue)
        self.mic = sr.Microphone()
        self.recog = sr.Recognizer()

    @staticmethod
    def reactivate():
        InteractionVoice.activated = True

    def received_grip_callback(self, user_in):
        try:
            # grip = self.recog.recognize_sphinx(user_in, language="grasp-cmd")
            # Not using voice model, just using basic en-US model
            grip = self.recog.recognize_sphinx(
                user_in, keyword_entries=InteractionVoice.known_grips
            )
            print("Sphinx thinks you said " + grip)
            self.queue.put(grip)
        except sr.UnknownValueError:
            print("Sphinx could not understand audio")
        except sr.RequestError as e:
            print("Sphinx error; {0}".format(e))

    def run(self):
        print("start Voice")
        while True:
            try:
                if InteractionVoice.activated:
                    with self.mic as src:
                        self.recog.adjust_for_ambient_noise(src)
                        sound = self.recog.listen(
                            src, timeout=None, phrase_time_limit=2.0
                        )
                    self.received_grip_callback(sound)
            except KeyboardInterrupt:
                break

    @staticmethod
    def deactivate():
        InteractionVoice.activated = False


if __name__ == "__main__":
    r = sr.Recognizer()
    with sr.Microphone() as source:
        r.adjust_for_ambient_noise(source)
        print("Say something!")
        audio = r.listen(source)
    try:
        print(
            "Sphinx thinks you said "
            + r.recognize_sphinx(audio, keyword_entries=InteractionVoice.known_grips)
        )
    except sr.UnknownValueError:
        print("Sphinx could not understand audio")
    except sr.RequestError as e:
        print("Sphinx error; {0}".format(e))
