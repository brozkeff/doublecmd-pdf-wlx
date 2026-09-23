---
status: "Superseded: by 0012-poppler-splash-default-and-backend-licensing.md"
date: 2026-09-22
---
# MuPDF License and Binary Distribution

## Context and Problem Statement

MuPDF is distributed by the Debian package used for the build under AGPL v3 or later, while the plugin's own source is licensed under EUPL 1.2. The project must not imply that MuPDF can be relicensed by the plugin author, especially when release binaries statically link its libraries.

## Decision Outcome

Keep the plugin source under EUPL 1.2 and identify MuPDF and its dependencies separately under their original licenses. Release binaries with complete corresponding source and third-party notices under the applicable compatible AGPL terms, or obtain a commercial MuPDF license when AGPL distribution obligations are not acceptable. The build records the complete static dependency closure rather than relying on accidental loader symbols.

This supersedes the initial Markdown implementation and release description in ADR 0003.
