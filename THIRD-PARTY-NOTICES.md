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

## ICC profile resources

The source tree includes unmodified base profiles and a generated CMYK-to-sRGB
device-link profile in `assets/icc/`. They are not currently embedded in or
loaded by the `.wlx` binaries; the current `mutool` command-line renderer does
not use them for fallback color management.

### eciCMYK v2

- File: `assets/icc/eciCMYK_v2.icc`
- Copyright embedded in profile: Heidelberger Druckmaschinen AG.
- Source: [ECI profile downloads](https://eci.org/doku.php?id=en:downloads).
- License: ECI makes the profile available with permission of Heidelberg; it
  may be used, embedded, exchanged, and shared without restriction. It may not
  be altered or sold without written permission from ECI. The profile file is
  distributed unchanged.

### sRGB IEC 61966-2.1

- File: `assets/icc/sRGB.icm`
- Copyright embedded in profile: Hewlett-Packard Company.
- License: the profile may be used, copied, and distributed without fee if it
  remains unchanged, including its HP copyright tag. Hewlett-Packard Company
  must not be used in advertising or publicity without prior written
  permission. The profile is provided as-is, without warranty.

### ECI CMYK v2 to sRGB device link

- File: `assets/icc/devicelink-eciCMYK_v2_to_sRGB.icc`
- Generated from the ECI CMYK v2 and sRGB profiles above with ArgyllCMS at
  low `-ql` quality; its `A2B0` LUT uses a 6-point CLUT.
- Copyright tag: `brozkeff`.
- Its short description contains no workstation path. Regenerate it with
  relative profile filenames to keep local paths out of the ICC metadata.

## Rust and GUI dependencies

The plugin dynamically links Qt5 and standard platform libraries. Rust crate
dependencies and their locked versions are listed in `Cargo.lock`; their
original licenses remain applicable. The project does not relicense Qt or Rust
dependencies under EUPL 1.2.
