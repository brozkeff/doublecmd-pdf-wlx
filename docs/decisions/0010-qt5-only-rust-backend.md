---
status: Accepted
date: 2026-09-23
---

# Qt5-only Rust Backend

## Context and Problem Statement

The Qt5 PDF viewer has now been manually verified in the official Qt5 Double
Commander package. No matching GTK3 Double Commander release is available, so
the GTK3 backend cannot be tested in the supported host and adds a second
binding, dependency set, and build path without a usable target.

## Decision Outcome

Keep Qt5 as the only Rust backend in the current project. Remove the GTK3
crate, dependencies, and build/test paths. Keep GTK3 support out of scope unless
a supported Double Commander GTK3 release becomes available and there is a
concrete need to validate it.

This supersedes the GTK3 secondary-backend portion of ADR 0009; its remaining
Qt5, Rust, and `mutool` decisions stay accepted.
