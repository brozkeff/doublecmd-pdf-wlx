---
status: Accepted
date: 2026-09-22
---
# Plugin Scope and Limits

## Context and Problem Statement

This is a small Linux WLX viewer for quick Markdown previews inside Double
Commander, not a complete Markdown application. It runs in the file manager
process, so unbounded input and active content are inappropriate.

## Decision Outcome

Support a safe Markdown subset for `.md`, `.markdown`, and `.mdown` on matching
Qt5, Qt6, GTK2, and GTK3 builds. Do not render raw HTML, load resources, open
links, or execute scripts; reject files over 4 MiB and let Double Commander
fall back to its normal viewer.
