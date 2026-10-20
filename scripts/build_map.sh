#!/usr/bin/env bash

set -eu

cargo run --release -p xtask --features build-map -- build-map \
    src/standard/shupai_map.rs \
    src/standard/zipai_map.rs \
    src/standard/wanzi_19_map.rs

cargo run --release -p xtask -- build-lut \
    src/standard/shupai_map.rs \
    src/standard/zipai_map.rs \
    src/standard/wanzi_19_map.rs \
    src/standard/lut.rs
