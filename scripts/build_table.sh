#!/usr/bin/env bash

cargo run --bin build-table --features build-file -- \
    src/standard/shupai_table.rs \
    src/standard/zipai_table.rs \
    src/standard/wanzi_19_table.rs
