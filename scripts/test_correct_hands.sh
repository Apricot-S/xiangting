#!/usr/bin/env bash

if [ -z "$1" ]; then
    echo "Usage: $0 <01-14>"
    exit 1
fi

if ! [[ "$1" =~ ^0[1-9]$|^1[0-4]$ ]]; then
    echo "Error: Argument must be between 01 and 14."
    exit 1
fi

cargo test correct_hands_$1 --release --features test -- --ignored
