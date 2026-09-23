# PDF WLX viewer for Double Commander

Standalone Linux WLX plugin for quick PDF previews in Double Commander. Select
a PDF and press F3 to render a scrollable first-page preview inside the
internal Lister.

Version 0.2.3 is the current Qt5-only patch of the Rust rewrite, manually
verified with the official Qt5 Double Commander package. GTK3 and Qt6 are not
supported targets.

The previous Pascal/Lazarus implementation remains available from Git tag
`v0.1.0` and commit `62e4ac2`.

Copyright (C) 2026 Martin Brozkeff Malec. The plugin source is licensed under
the [EUPL 1.2](LICENSE). MuPDF's `mutool` is a separate runtime program under
the GNU AGPL v3 or later; see
[THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md).

## Features and limits

- Detects `.pdf` files through the standard WLX ABI.
- Renders only the first page at 120 DPI through a `mutool` child process.
- Shows `Page 1 of N` when the PDF contains additional pages.
- Closes the active Lister window on Escape; the host's Q shortcut still works.
- Rejects invalid, encrypted, and empty documents. There is no page-count cap
  because the viewer renders only the first page.
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

Color profile files, including a CMYK-to-sRGB device link, are tracked under
`assets/icc/` for the planned profile-aware renderer. The current `mutool`
command-line path does not load them for fallback color management.

## Install

Select this plugin from a release or local build:

```text
build/pdf-wlx-x86_64-linux-qt5.wlx
```

In Double Commander, open **Configuration → Options → Plugins → WLX**, choose
**Add**, select the `.wlx` file, then open a PDF with F3.

## Build requirements

Install a current Rust toolchain, C++ compiler, `pkg-config`, MuPDF tools, and
Qt5 development files. On Ubuntu or Debian:

```sh
sudo apt install build-essential cargo pkg-config mupdf-tools qtbase5-dev
```

The Qt5 build uses Rust 2021, `cc`, and `pkg-config`. A small C++ shim creates
and destroys the viewer widgets and handles Escape because Qt has no stable C
ABI. The remaining WLX validation, rendering, temporary-file lifecycle, and
panic containment are implemented in Rust.

## Build and test

Build the Qt5 plugin:

```sh
./scripts/build.sh
```

Artifacts are written to the ignored `build/` directory:

```text
build/pdf-wlx-x86_64-linux-qt5.wlx
```

Run unit, lint, and ABI checks with:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
./scripts/smoke-test.sh
```

The smoke test verifies the exported WLX symbols, detection string, and plugin
version without constructing GUI widgets. A successful manual F3 preview in a
matching Double Commander build remains the definitive integration test.

Qt6 is not implemented; do not treat it as supported merely because the
shared Rust logic compiles.

## Release artifacts

Generated `.wlx` files and `build/` are not committed. Build release artifacts
from the tagged source, run the checks above, and upload the resulting files to
the corresponding GitHub release.

The plugin binary dynamically links Qt5 and the standard C++ runtime. It does
not contain MuPDF object code. Confirm release dependencies with:

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
Cargo.toml             Qt5 production crate
src/                   safe Rust WLX implementation and Qt5 C++ shim
tests/unit/            Rust unit test modules
scripts/build.sh       Qt5 release build wrapper
scripts/smoke-test.sh  Qt5 ABI and metadata smoke test
assets/icc/            ICC profile resources for planned color management
docs/decisions/        architecture and licensing decisions
```

See [LICENSE](LICENSE), [CHANGELOG.md](CHANGELOG.md), and the architecture
decisions under [docs/decisions](docs/decisions/).
