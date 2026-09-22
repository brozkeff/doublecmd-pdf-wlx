---
status: Accepted
date: 2026-09-22
---
# PDF Rendering with MuPDF

## Context and Problem Statement

The project is being remade from a Markdown Lister into a fast PDF previewer. PDF parsing and rendering should remain small, fast, and safe inside the Double Commander process while preserving the existing Qt and GTK WLX integration.

## Decision Outcome

Use the MuPDF C API through a small exception-safe C adapter. Render bounded page images at 120 DPI and place them in a scrollable native widget for the selected LCL widgetset. Reject documents that fail to open or exceed 128 pages and preserve the original PDF file.

This supersedes the Markdown scope in ADR 0002.
