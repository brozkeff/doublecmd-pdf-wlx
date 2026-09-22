#!/bin/sh

# Copyright (C) 2026 Martin Brozkeff Malec
# Licensed under the EUPL, Version 1.2.

set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
ARCH=$(uname -m)

test_plugin() {
  widgetset=$1
  plugin="$ROOT/build/pdf-wlx-$ARCH-linux-$widgetset.wlx"
  [ -f "$plugin" ] || {
    printf 'missing plugin: %s\n' "$plugin" >&2
    exit 1
  }
  nm -D --defined-only "$plugin" | grep -q ' ListLoad$'
  nm -D --defined-only "$plugin" | grep -q ' ListCloseWindow$'
  nm -D --defined-only "$plugin" | grep -q ' ListGetDetectString$'
  nm -D --defined-only "$plugin" | grep -q ' PdfWlxVersion$'

  PLUGIN="$plugin" WIDGETSET="$widgetset" python3 - <<'PY'
import ctypes
import os

plugin = ctypes.CDLL(os.environ["PLUGIN"])
buffer = ctypes.create_string_buffer(32)
plugin.ListGetDetectString(buffer, len(buffer))
assert buffer.value == b'EXT="PDF"', buffer.value

version = plugin.PdfWlxVersion
version.restype = ctypes.c_char_p
expected = f"0.2.0-rust-{os.environ['WIDGETSET']}".encode()
assert version() == expected, version()
print(f"ok: {os.path.basename(os.environ['PLUGIN'])}")
PY
}

[ "$#" -ge 1 ] || {
  printf 'Usage: %s {qt5|gtk3} [...]\n' "$0" >&2
  exit 2
}
for widgetset in "$@"; do
  test_plugin "$widgetset"
done
