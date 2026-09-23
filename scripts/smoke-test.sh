#!/bin/sh

# Copyright (C) 2026 Martin Brozkeff Malec
# SPDX-License-Identifier: EUPL-1.2 OR GPL-2.0-or-later OR AGPL-3.0-or-later

set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
ARCH=$(uname -m)
BACKEND=${1:-all}

case "$BACKEND" in
  poppler-splash)
    EXPECTED_VERSION=0.3.0-poppler-splash-qt5
    EXPECTED_LICENSE='GPL-2.0-or-later; project source also EUPL-1.2; see THIRD-PARTY-NOTICES.md'
    ;;
  mutool)
    EXPECTED_VERSION=0.3.0-mutool-qt5
    EXPECTED_LICENSE='AGPL-3.0-or-later; project source also EUPL-1.2; see THIRD-PARTY-NOTICES.md'
    ;;
  all)
    "$ROOT/scripts/smoke-test.sh" poppler-splash
    "$ROOT/scripts/smoke-test.sh" mutool
    exit 0
    ;;
  *)
    printf 'Usage: %s [all|poppler-splash|mutool]\n' "$0" >&2
    exit 2
    ;;
esac

[ "$#" -le 1 ] || {
  printf 'Usage: %s [all|poppler-splash|mutool]\n' "$0" >&2
  exit 2
}
PLUGIN="$ROOT/build/pdf-wlx-$ARCH-linux-qt5-$BACKEND.wlx"
[ -f "$PLUGIN" ] || {
  printf 'missing plugin: %s\n' "$PLUGIN" >&2
  exit 1
}
LICENSE_SIDECAR="$PLUGIN.license.txt"
[ -f "$LICENSE_SIDECAR" ] || {
  printf 'missing license sidecar: %s\n' "$LICENSE_SIDECAR" >&2
  exit 1
}

nm -D --defined-only "$PLUGIN" | grep -q ' ListLoad$'
nm -D --defined-only "$PLUGIN" | grep -q ' ListCloseWindow$'
nm -D --defined-only "$PLUGIN" | grep -q ' ListGetDetectString$'
nm -D --defined-only "$PLUGIN" | grep -q ' PdfWlxVersion$'

PLUGIN="$PLUGIN" LICENSE_SIDECAR="$LICENSE_SIDECAR" \
EXPECTED_VERSION="$EXPECTED_VERSION" EXPECTED_LICENSE="$EXPECTED_LICENSE" python3 - <<'PY'
import ctypes
import os

plugin = ctypes.CDLL(os.environ["PLUGIN"])
buffer = ctypes.create_string_buffer(32)
plugin.ListGetDetectString(buffer, len(buffer))
assert buffer.value == b'EXT="PDF"', buffer.value

version = plugin.PdfWlxVersion
version.restype = ctypes.c_char_p
expected = os.environ["EXPECTED_VERSION"].encode()
assert version() == expected, version()

license_name = plugin.PdfWlxLicense
license_name.restype = ctypes.c_char_p
assert license_name().decode() == os.environ["EXPECTED_LICENSE"]

with open(os.environ["LICENSE_SIDECAR"], encoding="utf-8") as license_file:
    sidecar = license_file.read()
assert f"Selected distribution license: {os.environ['EXPECTED_LICENSE'].split(';', 1)[0]}" in sidecar
print(f"ok: {os.path.basename(os.environ['PLUGIN'])}")
PY
