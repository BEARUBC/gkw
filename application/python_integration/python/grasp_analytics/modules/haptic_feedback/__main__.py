from .haptic_module import HapticFeedback
from src.grasp_analytics.data_generation.randomwalk_generator import RandomWalkGenerator
import json


def main():
    haptic_feedback = HapticFeedback()
    test_randomwalk = RandomWalkGenerator()
    test_randomwalk.generator()

    rw_list = test_randomwalk.randomwalk
    rw_dict = {"fsr_strengths": rw_list}
    haptic_feedback.run(json.dumps(rw_dict))


main()

# temporary --------------------------------------
# from src.haptic_feedback.randomwalk_generator import RandomWalkGenerator


# def main():
# randomwalk = RandomWalkGenerator()
# randomwalk.generator()
# randomwalk.plotter()


# main()
