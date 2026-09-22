---
status: Accepted
date: 2026-09-22
---
# Initial Implementation and Release

## Context and Problem Statement

The first release needs portable widgetset builds without requiring a Double
Commander source fork, while keeping the project small and easy to verify.

## Decision Outcome

Use standalone Free Pascal/Lazarus sources with minimal WLX declarations,
conditional native Qt/GTK backends, shell build and ABI smoke-test scripts,
and no GitHub Actions yet. Build binaries are released manually as GitHub
release assets; v0.1.0 covers the initial Qt5, Qt6, GTK2, and GTK3
implementations and their documented limits.
