#!/bin/sh

# Copyright (C) 2026 Martin Brozkeff Malec
# Licensed under the EUPL, Version 1.2.

set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
ARCH=$(uname -m)
PLUGIN="$ROOT/build/pdf-wlx-$ARCH-linux-qt5.wlx"

[ "$#" -eq 0 ] || {
  printf 'Usage: %s\n' "$0" >&2
  exit 2
}
[ -f "$PLUGIN" ] || {
  printf 'missing plugin: %s\n' "$PLUGIN" >&2
  exit 1
}

nm -D --defined-only "$PLUGIN" | grep -q ' ListLoad$'
nm -D --defined-only "$PLUGIN" | grep -q ' ListCloseWindow$'
nm -D --defined-only "$PLUGIN" | grep -q ' ListGetDetectString$'
nm -D --defined-only "$PLUGIN" | grep -q ' PdfWlxVersion$'

PLUGIN="$PLUGIN" python3 - <<'PY'
import ctypes
import os

plugin = ctypes.CDLL(os.environ["PLUGIN"])
buffer = ctypes.create_string_buffer(32)
plugin.ListGetDetectString(buffer, len(buffer))
assert buffer.value == b'EXT="PDF"', buffer.value

version = plugin.PdfWlxVersion
version.restype = ctypes.c_char_p
assert version() == b"0.2.2-rust-qt5", version()
print(f"ok: {os.path.basename(os.environ['PLUGIN'])}")
PY
