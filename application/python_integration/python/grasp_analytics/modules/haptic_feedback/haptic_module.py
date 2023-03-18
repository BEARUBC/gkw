from src.grasp_analytics.module import Module


class HapticFeedback(Module):
    def run(self, input_json: dict) -> dict:
        fsr_strengths = input_json["fsr_strengths"]
        vibration_strengths = [element * 2 for element in fsr_strengths]

        return {"coin_motor_vibration_strengths": vibration_strengths}
