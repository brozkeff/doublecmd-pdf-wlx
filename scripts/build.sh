#!/bin/sh

# Copyright (C) 2026 Martin Brozkeff Malec
# Licensed under the EUPL, Version 1.2.

# Build Rust WLX targets into build/.
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
BUILD_DIR="$ROOT/build"
ARCH=$(uname -m)

usage() {
  printf 'Usage: %s {qt5|qt6|gtk3|all}\n' "$0" >&2
  exit 2
}

build_target() {
  target=$1
  case "$target" in
    qt5)
      manifest="$ROOT/Cargo.toml"
      library=libpdf_wlx_qt5.so
      ;;
    gtk3)
      manifest="$ROOT/gtk3-wlx/Cargo.toml"
      library=libpdf_wlx_gtk3.so
      ;;
    qt6)
      printf 'Qt6 Rust WLX is not implemented yet\n' >&2
      return 1
      ;;
    *) usage ;;
  esac

  cargo build --release --manifest-path "$manifest"
  mkdir -p "$BUILD_DIR"
  output="$BUILD_DIR/pdf-wlx-$ARCH-linux-$target.wlx"
  cp "$ROOT/target/release/$library" "$output"
  printf 'built: %s\n' "$output"
}

[ "$#" -eq 1 ] || usage
case "$1" in
  qt5|qt6|gtk3) build_target "$1" ;;
  all)
    build_target qt5
    build_target gtk3
    ;;
  *) usage ;;
esac
