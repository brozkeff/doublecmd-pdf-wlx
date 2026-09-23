# Agent Notes

This repository contains a standalone Linux WLX PDF viewer for Double
Commander. The v0.3.0 release is implemented in Rust 2021 and supports the
manually verified Qt5 Double Commander package. Poppler Splash is the default
renderer; `mutool` is an optional compile-time backend. Qt5 is the only
supported target.

The root Cargo package contains the Qt5 WLX ABI and rendering orchestration.
Keep Qt interaction inside the narrow exception-safe C++ shim in
`src/qt5_shim.cpp`; all other logic should remain safe Rust where practical.
Every unsafe operation must have a local `SAFETY` comment, and no panic or C++
exception may cross the exported C ABI.

Both renderers show only the first page without a page-count ceiling. The
Poppler Splash backend parses and renders in memory through the C++ shim. The
`mutool` backend uses shell-free process arguments, private temporary
directories, synchronous image loading, and cleanup on every return path.

Use `./scripts/build.sh [all|poppler-splash|mutool]` and
`./scripts/smoke-test.sh [all|poppler-splash|mutool]`. The default builds both
backend artifacts with matching license sidecars. Outputs belong in the
ignored `build/` directory and must not be committed; upload them only as
release assets.

Before release, run `cargo fmt --all -- --check`, then run check, test, and
clippy with both the default Poppler feature and
`--no-default-features --features mutool`. Finish with the default
`./scripts/build.sh` and `./scripts/smoke-test.sh` to verify both artifacts.

Project-authored source is offered under EUPL 1.2, GPL-2.0-or-later, or
AGPL-3.0-or-later. The Poppler Splash artifact selects GPL-2.0-or-later; the
`mutool` artifact selects AGPL-3.0-or-later. Poppler and MuPDF retain their
original component licenses; read `THIRD-PARTY-NOTICES.md` before changing
runtime, release, or attribution details. Do not imply that renderer code is
relicensed.

Architecture decisions are in `docs/decisions/`. Accepted decision text is
not rewritten; supersede it with a new numbered ADR using minimal `status` and
`date` YAML metadata, an H1 title, and `Context and Problem Statement` plus
`Decision Outcome` sections.
