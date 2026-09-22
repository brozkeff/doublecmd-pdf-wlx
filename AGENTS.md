# Agent Notes

This repository contains a standalone Linux WLX PDF viewer for Double Commander. It is written in Free Pascal/Lazarus with a small dynamic C adapter around MuPDF and provides Qt5, Qt6, GTK2, and GTK3 backends.

The plugin source is EUPL 1.2. MuPDF and its dependencies retain their original licenses; read `THIRD-PARTY-NOTICES.md` before changing release or attribution details. A binary statically linking MuPDF is not EUPL-only and requires the applicable AGPL source and notice obligations, or a commercial MuPDF license.

Use `./scripts/build.sh {qt5|qt6|gtk2|gtk3|all}` and `./scripts/smoke-test.sh`. Build outputs and local compiler state are ignored. The build requires MuPDF headers; runtime users additionally need a shared `libmupdf.so`, which Ubuntu 22.04's `libmupdf-dev` package does not provide in this environment.

Architecture decisions are in `docs/decisions/`. Accepted decisions are not rewritten; supersede them with a new numbered ADR using minimal `status` and `date` YAML metadata, an H1 title, and `Context and Problem Statement` plus `Decision Outcome` sections.
