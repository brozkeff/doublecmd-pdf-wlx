---
status: "Superseded: by 0009-rust-rewrite-and-qt5-primary.md"
date: 2026-09-22
---

# MuTool Rendering Fallback

## Context and Problem Statement

Ubuntu 22.04 provides the MuPDF development package as static archives, but
does not provide the shared library required by the plugin's optional dynamic
MuPDF adapter. The installed `mupdf-tools` package does provide `mutool`, a
usable command-line renderer.

## Decision Outcome

The plugin first attempts rendering through the dynamically loaded MuPDF C
API. If that is unavailable or fails, it invokes `mutool draw` through
`TProcess`, passes arguments without a shell, renders at 120 DPI, and limits
the request to 128 pages. The generated PNG files are loaded by the native
widgetset viewer and removed after loading.

The runtime documentation must list `mupdf-tools` as a supported fallback
dependency. The plugin does not bundle `mutool` or MuPDF binaries.
