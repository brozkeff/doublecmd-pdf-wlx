# PDF WLX viewer for Double Commander

Standalone Linux WLX plugin for quick PDF previews in Double Commander. Select
a PDF and press F3 to render a scrollable first-page preview inside the
internal Lister.

Version 0.3.0 is a Qt5 plugin for fast first-page PDF previews. Poppler Splash
is the default renderer; the `mutool` renderer remains available as an
optional build. GTK3 and Qt6 are not supported targets.

The previous Pascal/Lazarus implementation remains available from Git tag
`v0.1.0` and commit `62e4ac2`.

Copyright (C) 2026 Martin Brozkeff Malec. Project-authored source is available
under EUPL 1.2, GPL-2.0-or-later, or AGPL-3.0-or-later. See [LICENSE](LICENSE)
and [LICENSES](LICENSES/) for the complete texts.

## Features and limits

- Detects `.pdf` files through the standard WLX ABI.
- Renders only the first page at 120 DPI. Poppler Splash renders in process;
  the `mutool` option uses a child process.
- Shows `Page 1 of N` when the PDF contains additional pages.
- Closes the active Lister window on Escape; the host's Q shortcut still works.
- Rejects invalid, encrypted, and empty documents. There is no page-count cap
  because the viewer renders only the first page.
- Uses in-memory pixels with Poppler; the alternate `mutool` backend removes
  rendered files after the native widget has loaded them.
- Enables Splash graphics and text antialiasing plus light text hinting.
- Contains panics and C++ exceptions at the plugin boundary.
- Keeps unsafe Rust and C++ limited to the documented WLX/native-widget shim.

The Poppler Splash backend renders page pixels in memory without preview files
on disk. The `mutool` backend writes a private temporary PNG and removes it
after the native widget loads it. Poppler parsing and rendering run in the C++
shim; Rust manages the `mutool` process when that backend is selected. If
parsing or rendering fails, the plugin returns an invalid WLX handle so the
file manager can use its normal fallback viewer.

## Runtime requirements

The recommended target is the official Qt5 Double Commander package. The
default Poppler backend needs the Qt5 and Poppler runtimes. On Ubuntu or Debian:

```sh
sudo apt install libqt5widgets5 libpoppler-qt5-1
```

To build or run the alternate `mutool` backend, also install `mupdf-tools`.
The plugin does not bundle MuPDF or Poppler libraries; the selected backend's
shared libraries must be available to Double Commander.

## Install

Choose the backend that matches your Double Commander runtime requirements:

```text
build/pdf-wlx-x86_64-linux-qt5-poppler-splash.wlx
build/pdf-wlx-x86_64-linux-qt5-mutool.wlx
```

In Double Commander, open **Configuration → Options → Plugins → WLX**, choose
**Add**, select the `.wlx` file, then open a PDF with F3.

## Build requirements

Install a current Rust toolchain, C++ compiler, `pkg-config`, Poppler Qt5
development files, and Qt5 development files. On Ubuntu or Debian:

```sh
sudo apt install build-essential cargo pkg-config libpoppler-qt5-dev qtbase5-dev
```

Install `mupdf-tools` as well to compile or run the alternate backend.

The Qt5 build uses Rust 2021, `cc`, and `pkg-config`. A small C++ shim creates
and destroys the viewer widgets and handles Escape because Qt has no stable C
ABI. Rust owns the WLX entry points and panic containment; the Poppler shim
uses Poppler's C++ API, while the `mutool` backend runs the renderer process
and manages its temporary-file lifecycle in Rust.

## Build and test

Build both backend variants (the default):

```sh
./scripts/build.sh
```

Build only one variant when desired:

```sh
./scripts/build.sh poppler-splash
./scripts/build.sh mutool
```

Artifacts are written to the ignored `build/` directory:

```text
build/pdf-wlx-x86_64-linux-qt5-poppler-splash.wlx
build/pdf-wlx-x86_64-linux-qt5-poppler-splash.wlx.license.txt
build/pdf-wlx-x86_64-linux-qt5-mutool.wlx
build/pdf-wlx-x86_64-linux-qt5-mutool.wlx.license.txt
```

The build also copies the project EUPL text and third-party notices into
`build/` for release packaging. Each `.wlx` has a sidecar with its selected
distribution license text.

Run unit, lint, and ABI checks with:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace --no-default-features --features mutool
cargo test --workspace --no-default-features --features mutool
cargo clippy --workspace --no-default-features \
  --features mutool --all-targets -- -D warnings
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

The Poppler binary dynamically links Qt5, Poppler, and the standard C++ runtime.
The `mutool` binary dynamically links Qt5 and the standard C++ runtime and
launches the separately installed `mutool` program. Neither binary bundles
MuPDF object code. Confirm dependencies with:

```sh
readelf -d build/pdf-wlx-x86_64-linux-qt5-poppler-splash.wlx | grep NEEDED
```

## Licensing

The source files carry this SPDX license expression:
`EUPL-1.2 OR GPL-2.0-or-later OR AGPL-3.0-or-later`. The repository keeps the
EUPL 1.2 text in [LICENSE](LICENSE), and provides the GPL and AGPL texts under
[LICENSES](LICENSES/). Each release build selects one distribution license and
reports it in the exported plugin metadata and matching `.license.txt` file.

| Artifact | License | Rendering dependency |
| --- | --- | --- |
| `*-poppler-splash.wlx` | `GPL-2.0-or-later` | Poppler dynamic link. |
| `*-mutool.wlx` | `AGPL-3.0-or-later` | External `mutool` process. |

The source remains available under EUPL 1.2. [EUPL Article 5 and its
Appendix](https://interoperable-europe.ec.europa.eu/licence/european-union-public-licence-version-12-eupl)
list GPL v2, GPL v3, and AGPL v3 as downstream-compatible licenses, so a larger
work may be distributed under those terms while preserving the licenses of its
individual components. The official [EUPL compatibility matrix](https://interoperable-europe.ec.europa.eu/collection/eupl/matrix-eupl-compatible-open-source-licences)
also marks dynamic linking to GPLv2 and GPLv3 components as compatible with
distribution under EUPL. The matrix describes its guidance as non-binding.

Poppler and MuPDF retain their own copyright and license terms. Distributors
must preserve the notices and provide the source obligations required by the
exact linked Poppler libraries or external `mutool` package. The current
`mutool` build does not bundle the external program.

## Layout

```text
Cargo.toml             Qt5 production crate and selectable renderer features
src/                   Rust WLX implementation and Qt5/Poppler C++ shims
tests/unit/            Rust unit test modules
scripts/build.sh       Qt5 build wrapper (builds both backends by default)
scripts/smoke-test.sh  ABI, version, and license-metadata checks
docs/decisions/        architecture and licensing decisions
LICENSES/              GPL-2.0-or-later and AGPL-3.0-or-later texts
```

See [LICENSE](LICENSE), [CHANGELOG.md](CHANGELOG.md), and the architecture
decisions under [docs/decisions](docs/decisions/).
