---
status: Accepted
date: 2026-09-22
---

# Rust Rewrite and Qt5 Primary Target

## Context and Problem Statement

The v0.1.0 Pascal/Lazarus implementation supported four widgetsets but carried
a larger toolchain and an in-process MuPDF integration boundary. The first Rust
prototype targeted GTK3, while official Linux Double Commander packages are
provided for GTK2, Qt5, and Qt6 rather than GTK3. A release target must be
testable against an official package without rebuilding Double Commander.

## Decision Outcome

Replace the production implementation with Rust 2021 and make Qt5 the primary
target. Keep WLX validation, process execution, temporary-file ownership, and
panic containment in safe Rust. Use a narrow exception-safe C++ shim only for
QWidget creation and destruction because Qt has no stable C ABI.

Invoke the external `mutool` executable for page counting and bounded rendering
instead of linking or loading MuPDF in the Double Commander process. Retain
GTK3 as a secondary Rust backend. Treat GTK2 and Qt6 as unimplemented until
matching Rust targets build and pass integration tests.

Generated `.wlx` files remain outside version control and are uploaded as
release assets after reproducible source builds and ABI checks. The v0.1.0
implementation remains recoverable from its Git tag and commit.

This supersedes ADRs 0003, 0004, 0007, and 0008.
