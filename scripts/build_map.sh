#!/usr/bin/env bash

cargo run --bin build-map --features build-map -- \
    src/standard/shupai_map.rs \
    src/standard/zipai_map.rs
