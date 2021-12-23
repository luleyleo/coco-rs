# Coco

Rust bindings for the COCO Numerical Black-Box Optimization Benchmarking Framework.

See https://github.com/numbbo/coco and https://numbbo.github.io/coco/.

# Building coco-sys

The COCO build process is a bit complicated and requires running `prebuild-coco.sh`. The resulting files are alread stored in Git in the `vendor/coco-prebuilt` folder. If COCO needs an update, this script must be run again, and the resulting files must be checked into Git. If there is no change in the COCO code base, a simple `cargo build` is enough.