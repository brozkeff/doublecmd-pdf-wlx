# Third-party notices

## MuPDF

This plugin dynamically loads MuPDF's C rendering library through `pdfmupdf.c`.

- Project: [MuPDF](https://mupdf.com/)
- Version used for the Ubuntu 22.04-compatible build: 1.19.0+ds1-2
- License: GNU AGPL v3 or later
- Source: [MuPDF source repository](https://github.com/ArtifexSoftware/mupdf) and [MuPDF source releases](https://mupdf.com/releases)
- Debian copyright and component notices: `/usr/share/doc/libmupdf-dev/copyright`

MuPDF is not relicensed under the EUPL by this project. The normal release
does not convey MuPDF binaries; it requires a compatible system shared library.
If a release bundles `libmupdf.so`, it must include the applicable AGPL notice,
license text, exact corresponding MuPDF source, and notices for its components.

## Other MuPDF dependencies

The build does not statically link MuPDF. A system MuPDF shared library brings
its own runtime dependencies, which may include MuJS, Gumbo, OpenJPEG, JBIG2Dec,
libjpeg, zlib, FreeType, libpng, and Brotli. Their original notices remain
applicable to whoever distributes that system library or a bundled copy.
