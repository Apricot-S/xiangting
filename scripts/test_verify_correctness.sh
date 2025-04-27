#!/usr/bin/env bash

if [ -z "$1" ]; then
    echo "Usage: NUM_THREADS=<1-N> $0 <01-14>"
    exit 1
fi

if ! [[ "$1" =~ ^0[1-9]$|^1[0-4]$ ]]; then
    echo "Error: Argument must be between 01 and 14."
    exit 1
fi

if (( $1 % 3 == 0 )); then
    echo "Error: Argument must not be divisible by 3."
    exit 1
fi

cargo test verify_correctness_$1 --release --features correctness -- --ignored
