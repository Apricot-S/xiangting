#!/usr/bin/env bash

cargo run --bin build-map --release --features build-map -- \
    src/standard/shupai_map.rs \
    src/standard/zipai_map.rs \
    src/standard/wanzi_19_map.rs
