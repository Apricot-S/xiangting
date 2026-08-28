#!/usr/bin/env bash

source "$(dirname "$0")/test_verify_common.sh"

run_verify_test correctness verify_necessary_tiles "$@"
