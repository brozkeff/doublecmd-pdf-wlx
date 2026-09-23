# Third-party notices

## Poppler Qt5 Splash

The optional `poppler-splash` build feature links to the system Poppler Qt5
interface and Poppler core libraries. Version 22.02.0 is installed for local
testing. The Poppler Qt5 interface header is licensed under the GNU GPL v2 or
later. The local package's copyright file identifies the core library as GPL
v2 or GPL v3 and Poppler contributions as GPL v2 or later. Other files may
carry different terms. Check the upstream and distribution copyright notices
for the exact libraries used. The plugin does not bundle Poppler libraries and
does not relicense Poppler under EUPL 1.2.

The `poppler-splash` release artifact is offered under GPL-2.0-or-later. The
project-authored source remains available under EUPL 1.2; the source SPDX
expression also offers GPL-2.0-or-later and AGPL-3.0-or-later. EUPL's
compatibility clause lists GPL v2 and v3 as downstream-compatible. The
[official matrix](https://interoperable-europe.ec.europa.eu/collection/eupl/matrix-eupl-compatible-open-source-licences)
also marks dynamic linking to GPLv2 and GPLv3 components as compatible with
distribution under EUPL. It describes this as guidance, not a guarantee.
Poppler retains its own license; preserve its notices and meet source
availability requirements for the exact libraries distributed.

- Project: [Poppler](https://poppler.freedesktop.org/).
- Qt5 API: [Poppler Qt5 documentation](https://poppler.freedesktop.org/api/qt5/).
- Tested version: 22.02.0 from the local Debian/Ubuntu package.
- Local package notices: `/usr/share/doc/libpoppler-qt5-dev/copyright`.
- Backend API: `Poppler::Document::SplashBackend` and
  `Poppler::Page::renderToImage()`.
- The Splash renderer enables Poppler graphics antialiasing, text
  antialiasing, text hinting, and slight text hinting.

## MuPDF tools

The `mutool` release artifact is offered under AGPL-3.0-or-later. The plugin
invokes the separately installed `mutool` executable to inspect and render PDF
documents. It does not bundle, statically link, dynamically load, or
redistribute MuPDF code. The source remains available under EUPL 1.2, and its
SPDX expression also permits GPL-2.0-or-later and AGPL-3.0-or-later.

- Project: [MuPDF](https://mupdf.com/)
- Tested version: 1.19.0+ds1-2
- License: GNU AGPL v3 or later
- Source: [MuPDF source repository](https://github.com/ArtifexSoftware/mupdf)
  and [MuPDF releases](https://mupdf.com/releases)
- Debian notices: `/usr/share/doc/mupdf-tools/copyright`

MuPDF and its dependencies retain their upstream licenses. The selected AGPL
license for this plugin artifact does not imply that MuPDF code is inside the
plugin or relicense MuPDF. Distributors who bundle `mutool` with the plugin
must separately provide all applicable AGPL source, license, copyright, and
component notices for the exact MuPDF binary they distribute. Installing
`mupdf-tools` from the operating system keeps that package's distribution and
notices under the system package manager.

## Rust and GUI dependencies

The plugin dynamically links Qt5 and standard platform libraries. Rust crate
dependencies and their locked versions are listed in `Cargo.lock`; their
original licenses remain applicable. The project does not relicense Qt or Rust
dependencies under EUPL 1.2.
