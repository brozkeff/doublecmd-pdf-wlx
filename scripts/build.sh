#!/bin/sh

# Copyright (C) 2026 Martin Brozkeff Malec
# Licensed under the EUPL, Version 1.2.

# Build the Qt5 Rust WLX plugin into build/.
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
BUILD_DIR="$ROOT/build"
ARCH=$(uname -m)

[ "$#" -eq 0 ] || {
  printf 'Usage: %s\n' "$0" >&2
  exit 2
}

cargo build --release --manifest-path "$ROOT/Cargo.toml"
mkdir -p "$BUILD_DIR"
output="$BUILD_DIR/pdf-wlx-$ARCH-linux-qt5.wlx"
cp "$ROOT/target/release/libpdf_wlx_qt5.so" "$output"
printf 'built: %s\n' "$output"
