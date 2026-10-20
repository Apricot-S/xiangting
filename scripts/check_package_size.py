#!/usr/bin/env python3
"""Keep the compressed crate below 9 MB, with headroom below crates.io's limit."""

from pathlib import Path
import sys

LIMIT = 9_000_000

if len(sys.argv) < 2:
    sys.exit("Usage: check_package_size.py <package.crate>...")

for argument in sys.argv[1:]:
    package = Path(argument)
    size = package.stat().st_size
    print(f"{package.name}: {size:,} bytes (limit {LIMIT:,})")
    if size > LIMIT:
        sys.exit(f"Package exceeds the {LIMIT:,}-byte distribution budget")
