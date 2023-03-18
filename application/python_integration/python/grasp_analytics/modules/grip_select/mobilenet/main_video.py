import cv2
from src.grasp_analytics.definitions import ROOT_PATH
from .objects import OBJECT_GRIP_MAP

# img = cv2.imread('cup.jpg')

cap = cv2.VideoCapture(0)
cap.set(3, 640)
cap.set(4, 480)

classFile = ROOT_PATH / "grip_select/mobilenet/mobilenet.names"
with open(classFile, "rt") as f:
    classNames = f.read().rstrip("\n").split("\n")

configPath = str(
    ROOT_PATH / "grip_select/mobilenet/ssd_mobilenet_v3_large_coco_2020_01_14.pbtxt"
)
weightsPath = str(ROOT_PATH / "grip_select/mobilenet/frozen_inference_graph.pb")

net = cv2.dnn_DetectionModel(weightsPath, configPath)
net.setInputSize(320, 320)
net.setInputScale(1.0 / 127.5)
net.setInputMean((127.5, 127.5, 127.5))
net.setInputSwapRB(True)

top_left = (240, 160)
bottom_right = (400, 320)

while True:
    success, img = cap.read()
    print(success)
    print(type(img))
    classIds, confs, bbox = net.detect(img, confThreshold=0.60)
    print(classIds, bbox)

    box_sizes = []

    i = 0

    if len(classIds) != 0:
        box_data = list(zip(classIds.flatten(), confs.flatten(), bbox))
        for classId, confidence, box in box_data:
            cv2.rectangle(img, box, color=(0, 255, 0), thickness=2)
            cv2.putText(
                img,
                classNames[classId - 1].upper(),
                (box[0] + 10, box[1] + 30),
                cv2.FONT_HERSHEY_SIMPLEX,
                1,
                (0, 255, 0),
                2,
            )

            box_offset = (box[0] + 1 / 2 * box[2], box[1] + 1 / 2 * box[3])
            box_sizes.append(box[2] * box[3])

            if (
                box_offset[0] > top_left[0]
                and box_offset[0] < bottom_right[0]
                and box_offset[1] > top_left[1]
                and box_offset[1] < bottom_right[1]
            ):
                cv2.rectangle(img, box, color=(0, 255, 255), thickness=2)
                cv2.putText(
                    img,
                    classNames[classId - 1].upper(),
                    (box[0] + 10, box[1] + 30),
                    cv2.FONT_HERSHEY_SIMPLEX,
                    1,
                    (0, 255, 255),
                    2,
                )
            i += 1

        # box_data = list(zip(classIds.flatten(), confs.flatten(), bbox, box_sizes))
        min_size = 100000
        selected_box = None

        for i in range(len(box_data)):
            box = box_data[i][2]
            box_offset = (box[0] + 1 / 2 * box[2], box[1] + 1 / 2 * box[3])
            if (
                box_sizes[i] < min_size
                and box_offset[0] > top_left[0]
                and box_offset[0] < bottom_right[0]
                and box_offset[1] > top_left[1]
                and box_offset[1] < bottom_right[1]
            ):
                selected_box = box_data[i]
                min_size = box_sizes[i]

        if selected_box:
            cv2.rectangle(img, selected_box[2], color=(255, 255, 0), thickness=2)
            cv2.putText(
                img,
                classNames[selected_box[0] - 1].upper(),
                (selected_box[2][0] + 10, selected_box[2][1] + 30),
                cv2.FONT_HERSHEY_SIMPLEX,
                1,
                (255, 255, 0),
                2,
            )
            cv2.putText(
                img,
                str(OBJECT_GRIP_MAP[classNames[selected_box[0] - 1]]),
                (selected_box[2][0] + 10, selected_box[2][1] + 60),
                cv2.FONT_HERSHEY_SIMPLEX,
                1,
                (255, 255, 0),
                2,
            )

        cv2.imshow("Output", img)
        cv2.waitKey(1)
