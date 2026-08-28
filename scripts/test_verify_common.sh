#!/usr/bin/env bash

run_verify_test() {
    if [ "$#" -ne 4 ]; then
        echo "Usage: $0 <01|02|04|05|07|08|10|11|13|14> <NUM_THREADS>"
        return 1
    fi

    local test_target="$1"
    local test_name="$2"
    local tile_count="$3"
    local num_threads="$4"

    if ! [[ "$num_threads" =~ ^[1-9][0-9]*$ ]]; then
        echo "Error: NUM_THREADS must be a positive integer."
        return 1
    fi

    if ! [[ "$tile_count" =~ ^0[1-9]$|^1[0-4]$ ]]; then
        echo "Error: Argument must be between 01 and 14."
        return 1
    fi

    if (( 10#$tile_count % 3 == 0 )); then
        echo "Error: Argument must not be divisible by 3."
        return 1
    fi

    NUM_THREADS="$num_threads" \
    cargo test --release --features correctness --test "$test_target" \
        "${test_name}_${tile_count}" -- --ignored --exact
}
