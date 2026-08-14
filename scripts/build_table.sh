#!/usr/bin/env bash

cargo run --release -p xtask -- build-table \
    src/standard/shupai_table.rs \
    src/standard/zipai_table.rs \
    src/standard/wanzi_19_table.rs
