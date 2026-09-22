# PDF WLX viewer for Double Commander

Standalone Linux WLX (Lister) plugin for quick F3 previews of PDF files in
Double Commander. It uses MuPDF through a runtime `dlopen`/`dlsym` adapter and
supports Qt5, Qt6, GTK2, and GTK3 builds.

Copyright (C) 2026 Martin Brozkeff Malec. The plugin source is licensed under
the [EUPL 1.2](LICENSE). MuPDF is a separate third-party component under the
[GNU AGPL v3 or later](https://www.gnu.org/licenses/agpl-3.0.html). See
[THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md) before distributing binaries.

## Usage

The plugin claims `*.pdf`. Select a PDF in Double Commander and press F3. The
internal Lister renders all pages at 120 DPI into native widgets, with a
scrollable page column. Files that cannot be opened, are encrypted, or exceed
128 pages are rejected so Double Commander can use its normal fallback.

If neither the shared MuPDF API nor `mutool` can render the document, the
plugin returns an invalid WLX handle and Double Commander uses its fallback
viewer.

## Runtime requirements

Install the runtime package matching the Double Commander widgetset:

- Qt5: `libqt5pas1` and the Qt5 runtime;
- Qt6: `libqt6pas6` and the Qt6 runtime;
- GTK2: `libgtk2.0-0`;
- GTK3: `libgtk-3-0`;
- all variants: either a shared MuPDF library exposing the MuPDF C API, or
  the `mutool` executable from `mupdf-tools`.

The plugin first tries the shared MuPDF API. If that library is unavailable or
incompatible, it runs `mutool draw` as a subprocess. The subprocess fallback
therefore needs `mupdf-tools` and `mutool` in the same `PATH` used to start
Double Commander. Installing the `mupdf` GUI alone is not sufficient.

Check the MuPDF library before installing the plugin:

```sh
ldconfig -p | grep libmupdf
```

The Ubuntu 22.04 package set used for development provides `libmupdf-dev` as
static development archives, not a runtime `libmupdf.so`. Installing
`libmupdf-dev` alone is therefore insufficient for the dynamic API path; use
`mupdf-tools` for the subprocess fallback. Use a distribution or vendor
package that provides a compatible shared library, or
build MuPDF as a shared library from the [official source](https://github.com/ArtifexSoftware/mupdf).
Keep its transitive runtime dependencies installed as reported by `ldd`.

For a locally installed library, for example:

```sh
LD_LIBRARY_PATH=/opt/mupdf/lib doublecmd
```

The plugin tries `libmupdf.so.25`, `libmupdf.so.1`, and `libmupdf.so`. The
soname and exported ABI are distribution-specific; use the same MuPDF major
ABI family used to build and test the plugin.

## Build requirements

On Ubuntu 22.04 (Jammy), install the compiler, Lazarus, headers, and matching
widgetset development packages:

```sh
sudo apt update
sudo apt install build-essential pkg-config fpc lazarus \
  libmupdf-dev mupdf-tools libgtk2.0-dev libgtk-3-dev \
  libqt5pas-dev libqt6pas6-dev
```

The Jammy-compatible development environment used Free Pascal 3.2.2,
Lazarus 4.8.0, `libmupdf-dev` and `mupdf-tools` 1.19.0+ds1-2,
`libgtk2.0-dev` 2.24.33-2ubuntu2.1, `libgtk-3-dev` 3.24.33-1ubuntu2.2,
`libqt5pas1` 4.2, and `libqt6pas6` plus `libqt6pas6-dev` 6.2.10.
Package revisions may change with Ubuntu updates; inspect them with
`dpkg-query -W`.

## Build and test

```sh
./scripts/build.sh qt5   # or qt6, gtk2, gtk3, all
./scripts/smoke-test.sh
```

Set `LAZARUS_DIR` when Lazarus is installed outside the default path. Binaries
are written to `build/pdf-wlx-x86_64-linux-<widgetset>.wlx`. Add the binary
matching Double Commander's widgetset under Options → Plugins → WLX.

The release binaries contain the plugin and dynamic loader only. They do not
contain MuPDF object code or a `libmupdf` ELF dependency:

```sh
readelf -d build/pdf-wlx-x86_64-linux-qt5.wlx | grep NEEDED
nm -D build/pdf-wlx-x86_64-linux-qt5.wlx | grep -E 'mupdf|fz_' || true
```

## AGPL compliance and EUPL compatibility

The plugin's own source remains EUPL 1.2. MuPDF remains AGPL-3+ and is not
relicensed by this project. The [MuPDF source repository](https://github.com/ArtifexSoftware/mupdf)
contains its source, `COPYING`, build instructions, and third-party component
notices. The [MuPDF release page](https://mupdf.com/releases) also states the
AGPL/commercial licensing options.

For the dynamic build, this repository does not convey MuPDF binaries. A
GitHub release should nevertheless include:

- this plugin source and its EUPL 1.2 `LICENSE`;
- this `THIRD-PARTY-NOTICES.md` file;
- a direct link to the exact MuPDF source/version used for testing;
- instructions naming the required system MuPDF shared library;
- source and license notices for any MuPDF modifications made by the user.

If a release later bundles `libmupdf.so`, its source, AGPL license, copyright
notices, and all dependency notices must be distributed as part of the same
release. The corresponding source must cover the exact bundled binary, not just
an unrelated newer MuPDF checkout. Do not describe a bundled release as
“EUPL-only”.

EUPL 1.2's compatibility mechanism allows compatible copyleft licensing such
as AGPL v3 for a combined work. It does not turn AGPL code into EUPL code, and
it does not remove AGPL source, notice, or redistribution requirements. The
safe interpretation is: keep this repository's original source under EUPL,
keep MuPDF under AGPL, and allow the applicable AGPL terms to govern any
combined distribution. Dynamic loading reduces what this project conveys and
is preferable for GitHub assets, but it is not a legal guarantee that a
designed runtime dependency is outside AGPL scope. Seek legal review or an
Artifex commercial license if that distinction matters commercially.

Useful primary references:

- [MuPDF source and AGPL notice](https://github.com/ArtifexSoftware/mupdf);
- [GNU AGPL v3 text](https://www.gnu.org/licenses/agpl-3.0.html);
- [MuPDF commercial licensing](https://artifex.com/licensing);
- [EUPL 1.2 text and compatibility information](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12).

## Layout

```text
pdf-wlx.lpr             WLX entry points and native Qt/GTK viewers
pdfmupdf.c              dynamic MuPDF adapter
sdk/                    minimal standalone WLX declarations
scripts/build.sh        widgetset build and dynamic-link build step
scripts/smoke-test.sh   ABI and detection checks
docs/decisions/         architecture and licensing decisions
```

See [LICENSE](LICENSE) and [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md).
