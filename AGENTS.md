# Agent Notes

This repository contains a standalone Linux WLX PDF viewer for Double
Commander. Version 0.2.3 is implemented in Rust 2021 and supports the manually
verified Qt5 Double Commander package. Qt5 is the only supported target.

The root Cargo package contains the Qt5 WLX ABI and rendering orchestration.
Keep Qt interaction inside the narrow exception-safe C++ shim in
`src/qt5_shim.cpp`; all other logic should remain safe Rust where practical.
Every unsafe operation must have a local `SAFETY` comment, and no panic or C++
exception may cross the exported C ABI.

PDF parsing is delegated to the external `mutool` process. Render only the
first page for the quick preview without a page-count ceiling. Preserve the
120-DPI limit, shell-free argument passing, private temporary directories,
synchronous image loading, and cleanup on every return path.

Use `./scripts/build.sh` and `./scripts/smoke-test.sh`. Outputs belong in the
ignored `build/` directory and must not be committed; upload them only as
release assets.

Run `cargo fmt --all -- --check`, `cargo check --workspace`,
`cargo test --workspace`, and
`cargo clippy --workspace --all-targets -- -D warnings` before release.

The source is EUPL 1.2. MuPDF and `mutool` retain their original licenses;
read `THIRD-PARTY-NOTICES.md` before changing runtime, release, or attribution
details. The plugin must not imply that external MuPDF code is relicensed.

Architecture decisions are in `docs/decisions/`. Accepted decision text is
not rewritten; supersede it with a new numbered ADR using minimal `status` and
`date` YAML metadata, an H1 title, and `Context and Problem Statement` plus
`Decision Outcome` sections.
