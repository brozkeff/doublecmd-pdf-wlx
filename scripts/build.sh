#!/bin/sh

# Copyright (C) 2026 Martin Brozkeff Malec
# SPDX-License-Identifier: EUPL-1.2 OR GPL-2.0-or-later OR AGPL-3.0-or-later

# Build both backend-specific Qt5 Rust WLX plugins by default.
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
BUILD_DIR="$ROOT/build"
ARCH=$(uname -m)
PACKAGE_VERSION=$(awk -F'"' '/^version = / { print $2; exit }' "$ROOT/Cargo.toml")
BACKEND=${1:-all}

case "$BACKEND" in
  all|poppler-splash|mutool) ;;
  *)
    printf 'Usage: %s [all|poppler-splash|mutool]\n' "$0" >&2
    exit 2
    ;;
esac

[ "$#" -le 1 ] || {
  printf 'Usage: %s [all|poppler-splash|mutool]\n' "$0" >&2
  exit 2
}

build_backend() (
  backend=$1
  case "$backend" in
    poppler-splash)
      distribution_license=GPL-2.0-or-later
      license_file="$ROOT/LICENSES/GPL-2.0-or-later.txt"
      ;;
    mutool)
      distribution_license=AGPL-3.0-or-later
      license_file="$ROOT/LICENSES/AGPL-3.0-or-later.txt"
      ;;
  esac

  cargo build --release --manifest-path "$ROOT/Cargo.toml" \
    --no-default-features --features "$backend"
  mkdir -p "$BUILD_DIR"
  cp "$ROOT/LICENSE" "$BUILD_DIR/LICENSE-EUPL-1.2.txt"
  cp "$ROOT/THIRD-PARTY-NOTICES.md" "$BUILD_DIR/THIRD-PARTY-NOTICES.md"
  output="$BUILD_DIR/pdf-wlx-$ARCH-linux-qt5-$backend.wlx"
  cp "$ROOT/target/release/libpdf_wlx_qt5.so" "$output"
  {
    printf 'Artifact: %s\n' "$(basename -- "$output")"
    printf 'Selected distribution license: %s\n' "$distribution_license"
    printf 'Corresponding source: https://github.com/brozkeff/doublecmd-pdf-wlx/tree/v%s\n' "$PACKAGE_VERSION"
    printf 'Project source is also available under EUPL-1.2.\n'
    printf 'Third-party components retain their original licenses; see THIRD-PARTY-NOTICES.md.\n\n'
    cat "$license_file"
  } > "$output.license.txt"
  printf 'built: %s\nlicense: %s\n' "$output" "$output.license.txt"
)

case "$BACKEND" in
  all)
    build_backend poppler-splash
    build_backend mutool
    ;;
  *)
    build_backend "$BACKEND"
    ;;
esac
