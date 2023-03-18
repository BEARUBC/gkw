import cv2

from .analyzer import MobileNetAnalyzer

cap = cv2.VideoCapture(0)
cap.set(3, 640)
cap.set(4, 480)

analyzer = MobileNetAnalyzer()

while True:
    success, img = cap.read()
    cv2.imshow("webcam", img)
    print(analyzer.analyze_image(img))
    cv2.waitKey(0)
