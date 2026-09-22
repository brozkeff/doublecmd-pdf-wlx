#!/bin/sh

# Copyright (C) 2026 Martin Brozkeff Malec
# Licensed under the EUPL, Version 1.2.

set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)

test_plugin() {
  plugin=$1
  [ -f "$ROOT/build/$plugin" ] || {
    printf 'missing plugin: %s\n' "$plugin" >&2
    exit 1
  }
  nm -D --defined-only "$ROOT/build/$plugin" | grep -q ' ListLoad$'
  nm -D --defined-only "$ROOT/build/$plugin" | grep -q ' ListCloseWindow$'
  nm -D --defined-only "$ROOT/build/$plugin" | grep -q ' ListGetDetectString$'
  nm -D --defined-only "$ROOT/build/$plugin" | grep -q ' PdfWlxVersion$'
  PLUGIN="$ROOT/build/$plugin" python3 - <<'PY'
import ctypes
import os

plugin = ctypes.CDLL(os.environ["PLUGIN"])
buffer = ctypes.create_string_buffer(2048)
plugin.ListGetDetectString(buffer, len(buffer))
expected = b'EXT="PDF"'
assert buffer.value == expected, buffer.value
license = plugin.PdfWlxLicense
license.restype = ctypes.c_char_p
assert b'EUPL 1.2' in license(), license()
version = plugin.PdfWlxVersion
version.restype = ctypes.c_char_p
assert version() == b'0.1.0', version()
print(f"ok: {os.path.basename(os.environ['PLUGIN'])}: {buffer.value.decode()}")
PY
}

test_plugin pdf-wlx-x86_64-linux-qt5.wlx
test_plugin pdf-wlx-x86_64-linux-qt6.wlx
test_plugin pdf-wlx-x86_64-linux-gtk2.wlx
test_plugin pdf-wlx-x86_64-linux-gtk3.wlx
