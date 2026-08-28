#!/usr/bin/env bash

run_verify_test() {
    if [ "$#" -ne 3 ]; then
        echo "Usage: NUM_THREADS=<1-N> $0 <01-14>"
        return 1
    fi

    local test_target="$1"
    local test_name="$2"
    local tile_count="$3"

    if ! [[ "$tile_count" =~ ^0[1-9]$|^1[0-4]$ ]]; then
        echo "Error: Argument must be between 01 and 14."
        return 1
    fi

    if (( 10#$tile_count % 3 == 0 )); then
        echo "Error: Argument must not be divisible by 3."
        return 1
    fi

    cargo test --release --features correctness --test "$test_target" \
        "${test_name}_${tile_count}" -- --ignored --exact
}
