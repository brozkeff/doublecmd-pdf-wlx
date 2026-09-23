---
status: Accepted
date: 2026-09-23
---
# Poppler Splash Default and Backend-Specific Licensing

## Context and Problem Statement

The v0.2.x viewer used an external `mutool` process. On the user's CMYK PDF,
MuPDF rendered quickly with smooth text but converted untagged CMYK colors
poorly. Poppler Splash produced colors close to the Cairo/Xreader rendering;
after enabling graphics antialiasing, text antialiasing, text hinting, and
slight text hinting, the user confirmed that its rendering quality is
excellent. The local five-run WLX harness measured a 209 ms median `ListLoad`
for Poppler Splash and 299 ms for `mutool`.

Poppler Qt5 and core libraries have GPL terms, while the project source is
EUPL-1.2. The EUPL compatibility clause and [official compatibility
matrix](https://interoperable-europe.ec.europa.eu/collection/eupl/matrix-eupl-compatible-open-source-licences)
permit compatible downstream licensing, but each build artifact must identify
its selected license and preserve component notices. The optional `mutool`
backend starts an external executable and does not link or bundle MuPDF code.

## Decision Outcome

Use Poppler Splash as the default renderer and retain `mutool` as an optional
compile-time backend. Both render only page 1 at 120 DPI, and Poppler keeps the
rendered page in memory. Enable the Poppler graphics/text antialiasing and
light-hinting flags confirmed by manual testing.

EUPL lists GPL v2, GPL v3, and AGPL v3 as compatible downstream licenses in
[Article 5 and its Appendix](https://interoperable-europe.ec.europa.eu/licence/european-union-public-licence-version-12-eupl).
Offer project-authored source under `EUPL-1.2 OR GPL-2.0-or-later OR
AGPL-3.0-or-later`. The repository's `LICENSE` remains the EUPL 1.2 text; the
`LICENSES/` directory carries the GPL and AGPL texts. Select a distribution
license per backend artifact:

- Poppler Splash `.wlx`: GPL-2.0-or-later.
- `mutool` `.wlx`: AGPL-3.0-or-later. This is the project's selected license
  for that build; it does not claim MuPDF code is linked into the plugin.

The build creates both artifacts by default and writes a matching sidecar with
the selected license text. Plugin metadata reports that same license. Poppler,
MuPDF, and their dependencies retain their own licenses and required notices.
Distributors must provide the source and notices required by the exact runtime
components they distribute.

This supersedes the default-renderer choice in ADR 0009 and the binary-license
packaging outcome in ADR 0005. The Qt5-only target and the separate-process
MuPDF implementation remain in effect.
