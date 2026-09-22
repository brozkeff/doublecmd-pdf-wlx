# Third-party notices

## MuPDF tools

This plugin invokes the separately installed `mutool` executable to inspect
and render PDF documents. It does not bundle, statically link, dynamically
load, or redistribute MuPDF code.

- Project: [MuPDF](https://mupdf.com/)
- Tested version: 1.19.0+ds1-2
- License: GNU AGPL v3 or later
- Source: [MuPDF source repository](https://github.com/ArtifexSoftware/mupdf)
  and [MuPDF releases](https://mupdf.com/releases)
- Debian notices: `/usr/share/doc/mupdf-tools/copyright`

MuPDF and its dependencies are not relicensed under the EUPL by this project.
Distributors who bundle `mutool` with the plugin must separately provide all
applicable AGPL source, license, copyright, and component notices for the exact
MuPDF binary they distribute. Installing `mupdf-tools` from the operating
system keeps that package's distribution and notices under the system package
manager.

## Rust and GUI dependencies

The plugin dynamically links the target GUI toolkit and standard platform
libraries. Rust crate dependencies and their locked versions are listed in
`Cargo.lock`; their original licenses remain applicable. The project does not
relicense Qt, GTK, or Rust dependencies under EUPL 1.2.
