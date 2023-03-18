import cv2
import numpy as np

from src.grasp_analytics.definitions import SETTINGS, ROOT_PATH
from src.grasp_analytics.utils import BoundingBox, Point

CROP_DIMS = tuple(SETTINGS["grip_select"]["crop_cnn"]["crop_dims"])


def _objectness_contours(img):
    objectness_model = cv2.saliency.StaticSaliencySpectralResidual_create()
    _, saliency_map = objectness_model.computeSaliency(np.float32(img))

    gray = np.array(saliency_map * 255).astype("uint8")
    # cv2.imshow("gray", gray)
    thresh = cv2.threshold(gray, 50, 255, cv2.THRESH_BINARY)[1]
    contours, heirarchy = cv2.findContours(
        thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE
    )
    return contours


def _get_best_bbox_from_contours(img, ctrs):
    img_center = Point(img.shape[0] // 2, img.shape[1] // 2)
    boxes = []
    for cntr in ctrs:
        x, y, w, h = cv2.boundingRect(cntr)
        centre_x = x + w // 2
        centre_y = y + h // 2
        max_dist_to_edge = max(w, h) // 2
        padding = 10

        r = max_dist_to_edge + padding

        bbox_center = Point(centre_y, centre_x)
        bbox = BoundingBox.from_center(bbox_center, r, r)
        img_bbox = BoundingBox.from_corners(0, 0, *img.shape[:2])
        if not img_bbox.contains_box(bbox):
            cv2.rectangle(img, (x, y), (x + w, y + h), (255, 0, 0), 2)
            continue
        heur = bbox_center.sqr_dist_to(img_center)

        # cv2.rectangle(img, (x, y), (x + w, y + h), (0, 0, 255), 2)

        boxes.append((bbox, heur))

    best_box_tuple = min(boxes, key=lambda box: box[1])
    return best_box_tuple[0]


def _crop_scale_image(img, box: BoundingBox):
    center = box.center
    d = box.w // 2
    cropped = img[
        int(center.x - d) : int(center.x + d), int(center.y - d) : int(center.y + d)
    ]
    return cv2.resize(cropped, CROP_DIMS)


def get_best_obj_img(img):
    contours = _objectness_contours(img)
    best_box = _get_best_bbox_from_contours(img, contours)
    return _crop_scale_image(img, best_box)


if __name__ == "__main__":
    img_path = (
        ROOT_PATH / SETTINGS["grip_select"]["data_dir"] / "images/cup/cup_001.jpg"
    )
    im = cv2.imread(str(img_path))
    best_obj_img = get_best_obj_img(im)
    cv2.imshow("final", best_obj_img)
    cv2.waitKey(1)
