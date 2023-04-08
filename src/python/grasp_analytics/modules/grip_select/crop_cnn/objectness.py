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
        thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_NONE
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

def largest_contour_box(img, contours):
    """
    Delete this function soon
    """
    largest_contiour = max(contours, key=cv2.contourArea)

    x,y,w,h = cv2.boundingRect(largest_contiour)

    cv2.rectangle(img, (x,y), (x+w, y+h),(0, 255, 0),2)

    return (x,y,w,h)

def show_bounding(img):
    # image = cv2.imread(img)
    image = img
    gray_image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    _, thresh_image = cv2.threshold(gray_image, 127, 255, cv2.THRESH_BINARY)
    contours, _ = cv2.findContours(thresh_image, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    x, y, w, h = largest_contour_box(image, contours)
    print(x,y,w,h)
    cv2.rectangle(image, (x, y), (x + w, y + h), (0, 255, 0), 2)
    cv2.imshow('Image with Bounding Box', image)
    cv2.waitKey(0)
    cv2.destroyAllWindows()




def maybe_best_obj_image(img):
    """
    Delete this function too
    """
    all_contours = _objectness_contours(img)
    largest_contour = largest_contour_box(img, all_contours)
    return _crop_scale_image(img, largest_contour)





if __name__ == "__main__":
    img_path = (
        ROOT_PATH / SETTINGS["grip_select"]["data_dir"] / "images/apple.jpg"
    )
    im = cv2.imread(str(img_path))


    best_obj_img = get_best_obj_img(im)
    contours = _objectness_contours(str(img_path))
    max(contours, key=cv2.contourArea)
    # print()
    # largest_contour = largest_contour_box(im, contours)
    cv2.drawContours(im, contours, -1, (0, 255, 0))
    # largest_contour = max(contours, key=cv2.contourArea)
    # cv2.drawContours(im, largest_contour, -1, (0, 255, 0))
    # show_bounding(im)
    cv2.imwrite(str(ROOT_PATH / SETTINGS["grip_select"]["data_dir"] / "images/apple_contours.JPG"),im)# best_obj_img)
    cv2.imwrite(str(ROOT_PATH / SETTINGS["grip_select"]["data_dir"] / "images/apple_out_out.JPG"), contours)# best_obj_img)
    # cv2.waitKey(1)
    # __________
    # best_image = delete_later(im
    #)
    # print(type(best_image))
    # cv2.imshow("Image", im)