import cv2

img = cv2.imread("img_8.png")

classFile = "mobilenet.names"
with open(classFile, "rt") as f:
    classNames = f.read().rstrip("\n").split("\n")

configPath = "ssd_mobilenet_v3_large_coco_2020_01_14.pbtxt"
weightsPath = "frozen_inference_graph.pb"

net = cv2.dnn_DetectionModel(weightsPath, configPath)
net.setInputSize(320, 320)
net.setInputScale(1.0 / 127.5)
net.setInputMean((127.5, 127.5, 127.5))
net.setInputSwapRB(True)

height, width, channels = img.shape
top_left = (240, 160)
bottom_right = (400, 320)
cv2.rectangle(img, top_left, bottom_right, (0, 0, 255), thickness=1)

classIds, confs, bbox = net.detect(img, confThreshold=0.5)
box_sizes = []
box_data = zip(classIds.flatten(), confs.flatten(), bbox)
i = 0
for classId, confidence, box in box_data:

    # cv2.rectangle(img, tuple(squareBox_top_left), tuple(squareBox_bot_right), color=(0, 255, 0), thickness=2)
    # cv2.putText(img, classNames[classId - 1].upper(), (box[0] + 10, box[1] + 30),
    #             cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 255, 0), 2)

    box_offset = (box[0] + 1 / 2 * box[2], box[1] + 1 / 2 * box[3])
    box_sizes.append(box[2] * box[3])

    # if (box_offset[0] > top_left[0] and box_offset[0] < bottom_right[0] and box_offset[1] > top_left[1] and box_offset[
    #     1] < bottom_right[1]):
    #     cv2.rectangle(img, box, color=(0, 255, 255), thickness=2)
    #     cv2.putText(img, classNames[classId - 1].upper(), (box[0] + 10, box[1] + 30),
    #                 cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 255, 255), 2)

    # i += 1

box_data = list(zip(classIds.flatten(), confs.flatten(), bbox, box_sizes))
min_size = 100000
selected_box = None

for i in range(len(box_data)):
    box = box_data[i][2]
    box_offset = (box[0] + 1 / 2 * box[2], box[1] + 1 / 2 * box[3])
    if (
        box_sizes[i] < min_size
        and top_left[0] < box_offset[0] < bottom_right[0]
        and top_left[1] < box_offset[1] < bottom_right[1]
    ):
        selected_box = box_data[i]
        min_size = box_sizes[i]
        square_box = selected_box[2]

        x_diff = square_box[2]
        y_diff = square_box[3]

        squareBox_top_left_tup = (square_box[0], square_box[1])
        squareBox_bot_right_tup = (
            square_box[0] + square_box[2],
            square_box[1] + square_box[3],
        )

        squareBox_top_left = list(squareBox_top_left_tup)
        squareBox_bot_right = list(squareBox_bot_right_tup)

        if x_diff == y_diff:
            square_box = box
        elif x_diff > y_diff:
            squareBox_top_left[1] = int(square_box[1] - (x_diff - y_diff) / 2)
            squareBox_bot_right[1] = int(
                square_box[1] + square_box[3] + (x_diff - y_diff) / 2
            )
        else:
            squareBox_top_left[0] = int(square_box[0] - (y_diff - x_diff) / 2)
            squareBox_bot_right[0] = int(
                square_box[0] + square_box[2] + (y_diff - x_diff) / 2
            )

cv2.rectangle(
    img,
    tuple(squareBox_top_left),
    tuple(squareBox_bot_right),
    color=(255, 255, 0),
    thickness=2,
)
cv2.putText(
    img,
    classNames[selected_box[0] - 1].upper(),
    (selected_box[2][0] + 10, selected_box[2][1] + 30),
    cv2.FONT_HERSHEY_SIMPLEX,
    1,
    (255, 255, 0),
    2,
)

cv2.imshow("Output", img)
cv2.waitKey(0)
