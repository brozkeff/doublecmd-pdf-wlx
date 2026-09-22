#!/bin/sh

# Copyright (C) 2026 Martin Brozkeff Malec
# Licensed under the EUPL, Version 1.2.

set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
LAZARUS_DIR=${LAZARUS_DIR:-/usr/share/lazarus/4.8.0}
PCP=${LAZARUS_PCP:-"$ROOT/.lazarus-config"}
LINK_DIR="$ROOT/.build-libs"
BUILD_DIR="$ROOT/build"

usage() {
  printf 'Usage: %s {qt5|qt6|gtk2|gtk3|all}\n' "$0" >&2
  exit 2
}

[ "$#" -eq 1 ] || usage

build_one() {
  widgetset=$1
  mkdir -p "$LINK_DIR"
  mkdir -p "$BUILD_DIR"
  cc -fPIC -O2 -c "$ROOT/pdfmupdf.c" -o "$ROOT/pdfmupdf.o"
  ar rcs "$BUILD_DIR/libpdfmupdf.a" "$ROOT/pdfmupdf.o"
  case "$widgetset" in
    qt5)
      if [ ! -e "$LINK_DIR/libQt5Pas.so" ]; then
        versioned=$(ldconfig -p 2>/dev/null | awk '/libQt5Pas\.so\.1 / { print $NF; exit }')
        if [ -n "$versioned" ]; then
          ln -sf "$versioned" "$LINK_DIR/libQt5Pas.so"
        fi
      fi
      ;;
    qt6)
      if [ ! -e "$LINK_DIR/libQt6Pas.so" ]; then
        versioned=$(ldconfig -p 2>/dev/null | awk '/libQt6Pas\.so\.6 / { print $NF; exit }')
        if [ -n "$versioned" ]; then
          ln -sf "$versioned" "$LINK_DIR/libQt6Pas.so"
        fi
      fi
      ;;
  esac
  lazbuild \
    --pcp="$PCP" \
    --scp=/etc/lazarus \
    --lazarusdir="$LAZARUS_DIR" \
    --ws="$widgetset" \
    --opt="-k-L$LINK_DIR -k-L$BUILD_DIR -k--whole-archive -k-lpdfmupdf -k--no-whole-archive -k-ldl" \
    "$ROOT/pdf-wlx.lpi"
  strip --strip-unneeded "$BUILD_DIR/pdf-wlx-$(uname -m)-linux-$widgetset.wlx"
}

case "$1" in
  qt5|qt6|gtk2|gtk3) build_one "$1" ;;
  all)
    failed=0
    for widgetset in qt5 qt6 gtk2 gtk3; do
      if ! build_one "$widgetset"; then
        printf 'warning: %s build unavailable or failed\n' "$widgetset" >&2
        failed=1
      fi
    done
    exit "$failed"
    ;;
  *) usage ;;
esac
