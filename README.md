# PDF WLX viewer for Double Commander

Standalone Linux WLX plugin for quick PDF previews in Double Commander. Select
a PDF and press F3 to render a scrollable column of pages inside the internal
Lister.

Version 0.2.0 is a Rust rewrite. The Qt5 backend is the primary implementation
and has been manually verified with the official Qt5 Double Commander package.
A GTK3 backend is also included, but official Double Commander packages do not
currently provide a matching GTK3 build. Qt6 is planned but not implemented.

The previous Pascal/Lazarus implementation remains available from Git tag
`v0.1.0` and commit `62e4ac2`.

Copyright (C) 2026 Martin Brozkeff Malec. The plugin source is licensed under
the [EUPL 1.2](LICENSE). MuPDF's `mutool` is a separate runtime program under
the GNU AGPL v3 or later; see
[THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md).

## Features and limits

- Detects `.pdf` files through the standard WLX ABI.
- Renders pages at 120 DPI through a `mutool` child process.
- Rejects invalid, encrypted, empty, and over-128-page documents.
- Uses private temporary directories and removes rendered pages after native
  widgets have loaded them.
- Contains panics and C++ exceptions at the plugin boundary.
- Keeps unsafe Rust and C++ limited to the documented WLX/native-widget shim.

PDF parsing happens in the `mutool` child process rather than inside Double
Commander. If rendering fails, the plugin returns an invalid WLX handle so the
file manager can use its normal fallback viewer.

## Runtime requirements

The recommended target is the official Qt5 Double Commander package. Install
the matching Qt5 runtime and MuPDF tools. On Ubuntu or Debian:

```sh
sudo apt install mupdf-tools libqt5widgets5
```

`mutool` must be available in the `PATH` inherited by Double Commander. The
plugin does not bundle or dynamically load MuPDF libraries.

The optional GTK3 plugin additionally requires the GTK3 runtime and a GTK3
build of Double Commander. It cannot be loaded into GTK2, Qt5, or Qt6 builds.

## Install

Use the plugin matching the Double Commander widgetset. For the supported Qt5
package, select this file from a release or local build:

```text
build/pdf-wlx-x86_64-linux-qt5.wlx
```

In Double Commander, open **Configuration → Options → Plugins → WLX**, choose
**Add**, select the `.wlx` file, then open a PDF with F3.

## Build requirements

Install a current Rust toolchain, C++ compiler, `pkg-config`, MuPDF tools, and
the development package for the selected widgetset. On Ubuntu or Debian:

```sh
sudo apt install build-essential cargo pkg-config mupdf-tools qtbase5-dev
```

For the optional GTK3 target, also install:

```sh
sudo apt install libgtk-3-dev
```

The Qt5 build uses Rust 2021, `cc`, and `pkg-config`. A small C++ shim performs
only QWidget creation and destruction because Qt has no stable C ABI. The
remaining WLX validation, rendering, temporary-file lifecycle, and panic
containment are implemented in Rust.

## Build and test

Build one target or all currently implemented targets:

```sh
./scripts/build.sh qt5
./scripts/build.sh gtk3
./scripts/build.sh all
```

Artifacts are written to the ignored `build/` directory:

```text
build/pdf-wlx-x86_64-linux-qt5.wlx
build/pdf-wlx-x86_64-linux-gtk3.wlx
```

Run unit, lint, and ABI checks with:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
./scripts/smoke-test.sh qt5 gtk3
```

The smoke test verifies the exported WLX symbols, detection string, and plugin
version without constructing GUI widgets. A successful manual F3 preview in a
matching Double Commander build remains the definitive integration test.

`./scripts/build.sh qt6` intentionally fails with a clear message until the
Qt6 shim exists. Do not treat Qt6 as implemented merely because the shared
Rust logic compiles.

## Release artifacts

Generated `.wlx` files and `build/` are not committed. Build release artifacts
from the tagged source, run the checks above, and upload the resulting files to
the corresponding GitHub release.

The plugin binary dynamically links the selected GUI toolkit and the standard
C++ runtime for Qt5. It does not contain MuPDF object code. Confirm release
dependencies with:

```sh
readelf -d build/pdf-wlx-x86_64-linux-qt5.wlx | grep NEEDED
```

## Licensing

The repository's original source is EUPL 1.2. The plugin invokes the external
`mutool` executable but does not distribute, link, or load MuPDF code. Anyone
redistributing MuPDF or `mutool` alongside the plugin must comply with the
applicable AGPL and third-party notice obligations. See
[THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md) for details.

## Layout

```text
Cargo.toml             Qt5 production crate and workspace root
src/                   safe Rust WLX implementation and Qt5 C++ shim
gtk3-wlx/              secondary GTK3 crate
scripts/build.sh       widgetset build wrapper
scripts/smoke-test.sh  ABI and metadata smoke tests
docs/decisions/        architecture and licensing decisions
```

See [LICENSE](LICENSE), [CHANGELOG.md](CHANGELOG.md), and the architecture
decisions under [docs/decisions](docs/decisions/).
