import json
import random
from pathlib import Path
from typing import List, Dict, Set

import numpy as np
import torch
from torchvision.datasets.folder import default_loader

from src.grasp_analytics.modules.grip_select.mobilenet import objects
from torchvision.datasets import VisionDataset


def corner_bbox_from_dim(x, y, w, h):
    return x, y, x + w, y + h

def collate_fn(batch):
    return tuple(zip(*batch))


class CocoDetection(VisionDataset):
    """`MS Coco Detection <http://mscoco.org/dataset/#detections-challenge2016>`_ Dataset.

    Args:
        root (string): Root directory where images are downloaded to.
        annFile (string): Path to json annotation file.
        transform (callable, optional): A function/transform that  takes in an PIL image
            and returns a transformed version. E.g, ``transforms.ToTensor``
        target_transform (callable, optional): A function/transform that takes in the
            target and transforms it.
    """

    def make_targets(self, annotations: List[Dict]):
        boxes = []
        areas = []
        labels = []
        crowds = []
        image_id: int = np.nan

        for idx, annotation in enumerate(annotations):
            if idx == 0:
                image_id = annotation["image_id"]
            else:
                assert annotation["image_id"] == image_id

            bbox = annotation["bbox"]

            bbox = list(corner_bbox_from_dim(*bbox))
            assert 0 <= bbox[0] < bbox[2]
            assert 0 <= bbox[1] < bbox[3]
            if abs(bbox[3] - bbox[1]) <= 0.5 or abs(bbox[2] - bbox[0]) <= 0.5:
                continue

            assert annotation["area"] > 0

            l: int = annotation["category_id"]

            # Depending on the split, classes that have not yet been seen should be masked out of the ground truth
            # labels to demonstrate background semantic shift with the introduction of new classes
            if isinstance(self._class_mask, list) and l not in self._class_mask:
                del annotations[idx]
                continue

            labels.append(l)
            boxes.append(bbox)
            areas.append(annotation["area"])
            crowds.append(0)

        box_tensor = torch.Tensor(boxes).to(torch.int64)
        label_tensor = torch.Tensor(labels).to(torch.int64)

        assert len(labels) == len(boxes)
        if len(labels) == 0:
            box_tensor = torch.empty((0, 4), dtype=torch.float32)
            label_tensor = torch.empty((0, 4), dtype=torch.int64)

        return {
            "boxes": box_tensor,
            "labels": label_tensor,
            "image_id": torch.Tensor([image_id]).to(torch.int64),
        }

    def _parse_annotations(self, annfile: Path):
        with open(annfile, "r") as f:
            annots = json.load(f)

            new_cats = [
                {"supercategory": "grip", "id": 0, "name": "tip"},
                {"supercategory": "grip", "id": 1, "name": "lateral"},
                {"supercategory": "grip", "id": 2, "name": "tripod"},
                {"supercategory": "grip", "id": 3, "name": "spherical"},
                {"supercategory": "grip", "id": 4, "name": "power"},
                {"supercategory": "grip", "id": 5, "name": "extension"}
            ]

            temp_annots = annots['annotations'].copy()
            mask = [category["name"] in objects.OBJECT_GRIP_MAP.keys() for category in annots["categories"]]
            masked_cat = [cat for cat, msk in zip(annots["categories"], mask) if msk]

            id_map = dict()
            name_map = dict()
            old_name_map = {x["name"]: x["id"] for x in masked_cat}
            for k, v in objects.OBJECT_GRIP_MAP.items():
                old_name = k
                if old_name in old_name_map:
                    old_id = old_name_map[old_name]
                    id_map[old_id] = int(objects.OBJECT_GRIP_MAP[old_name])
                    name_map[old_name] = objects

            annots["annotations"] = []
            for annot in temp_annots:
                if annot["category_id"] not in id_map:
                    continue

                annot["category_id"] = id_map[annot["category_id"]]
                annots["annotations"].append(annot)
            annots["categories"] = new_cats
            return annots



    def __init__(self, frame_path: Path, instances_path: Path,
                 class_mask=None,
                 shuffle=False,
                 transforms=None,
                 loader=default_loader):

        frames =

        self.__targets = None
        if transforms is None:
            transforms = list()
        self._transforms = transforms
        self.loader = loader

        self._class_mask = None
        if isinstance(class_mask, list):
            self._class_mask = class_mask.copy()
        self.__labels = None

        self.frames = frames
        self._class_names = []

        zipped = list(zip(self.frames, self._annotations))
        if shuffle:
            random.shuffle(zipped)

        # Removing datapoints with no objects in the scene will introduce bias.
        # TODO: Look into the cause of removing this causing an error.
        zipped = [x for x in zipped if len(x[1]) > 0]
        self.frames, self._annotations = tuple(zip(*zipped))

    def __repr__(self):
        fmt_str = 'Dataset ' + self.__class__.__name__ + '\n'
        fmt_str += '    Number of datapoints: {}\n'.format(self.__len__())
        fmt_str += '    Root Location: {}\n'.format(self.root)
        tmp = '    Transforms (if any): '
        fmt_str += '{0}{1}\n'.format(tmp, self.transform.__repr__().replace('\n', '\n' + ' ' * len(tmp)))
        tmp = '    Target Transforms (if any): '
        fmt_str += '{0}{1}'.format(tmp, self.target_transform.__repr__().replace('\n', '\n' + ' ' * len(tmp)))
        return fmt_str

    @property
    def targets(self) -> List[Dict[str, torch.Tensor]]:
        if self.__targets is None:
            self.__targets = [self.make_targets(self._annotations[i]) for i in range(len(self))]
        return self.__targets

    @property
    def labels(self) -> Set[int]:
        if self.__labels is None:
            self.__labels = set()
            for t in self.targets:
                self.__labels = self.__labels.union(set(t["labels"].tolist()))
        return self.__labels

    def __getitem__(self, idx):
        target = self.targets[idx]
        img_path = self.frames[idx]
        img = self.loader(str(img_path))

        for t in self._transforms:
            if t.needs_targets:
                img, target = t(img, target)
            else:
                img = t(img)

        return img, target

    def __len__(self):
        return len(self.frames)

    @property
    def class_names(self):
        return []
