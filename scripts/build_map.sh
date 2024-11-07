#!/usr/bin/env bash

start_time=$(date +%s)

cargo run --bin build-map --release --features build-map -- \
    src/standard/shupai_map.rs \
    src/standard/zipai_map.rs \
    src/standard/wanzi_19_map.rs

end_time=$(date +%s)
execution_time=$((end_time - start_time))

echo "Execution time: $execution_time seconds"
