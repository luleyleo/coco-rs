#!/usr/bin/env bash

cd vendor/coco
python3 do.py build-c

cd ..
mkdir -p coco-prebuilt
cp coco/code-experiments/build/c/coco.* coco-prebuilt/
cp coco/code-experiments/src/coco_internal.h coco-prebuilt/
