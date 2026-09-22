---
status: "Superseded: by 0009-rust-rewrite-and-qt5-primary.md"
date: 2026-09-22
---
# Dynamic MuPDF Binding

## Context and Problem Statement

The previous build statically linked MuPDF and its transitive dependencies. That made the WLX binary self-contained but also made every binary distribution carry the MuPDF AGPL work and its third-party notices. Ubuntu 22.04's development package does not provide the shared runtime needed by a dynamic build, so runtime requirements must be explicit.

## Decision Outcome

Build only the plugin and a small C adapter, then resolve MuPDF with `dlopen` and `dlsym` from a compatible system shared library at runtime. GitHub assets will not bundle MuPDF. Releases will retain EUPL source notices, AGPL dependency links, exact source/version guidance, and a clear statement that dynamic binding reduces redistribution scope but is not a legal guarantee outside AGPL coverage. This supersedes ADR 0005 and confirms the option proposed in ADR 0006.
