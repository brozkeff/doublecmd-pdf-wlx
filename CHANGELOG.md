# Changelog

<!-- markdownlint-configure-file {"MD024": {"siblings_only": true}} -->

## [v0.2.2] - 2026-09-23

### Changed

- Keep Qt5 as the sole Rust backend and remove the unsupported GTK3 crate,
  dependency, and build/test paths.
- Move Rust unit test bodies from `src/lib.rs` into `tests/unit/`.
- Refresh the tracked CMYK-to-sRGB device link from the path-free ArgyllCMS
  output in project `tmp/`.

### Fixed

- Close the active Qt5 Lister window on Escape while deferring host teardown
  until the key event has returned.
- Bump the plugin package and exported version metadata to 0.2.2.

### Notes

- Restart Double Commander after replacing the WLX file so it loads the updated
  plugin instead of the copy already loaded in the running process.

## [v0.2.1] - 2026-09-22

### Changed

- Render only page 1 for a faster multipage PDF preview in both backends.
- Show a `Page 1 of N` indicator when the document has more than one page.
- Move the ECI CMYK v2 and sRGB profile resources into `assets/icc/` for the
  planned profile-aware renderer.
- Add the supplied CMYK-to-sRGB device link after removing its workstation path
  from the description metadata; the color-transform table is unchanged.
- Bump both plugin backends and their exported version metadata to 0.2.1.

## [v0.2.0] - 2026-09-22

### Added

- Rewrote the plugin in Rust 2021 with explicit, documented FFI boundaries.
- Added a Qt5 backend using a narrow exception-safe C++ QWidget shim.
- Added bounded out-of-process PDF rendering through `mutool`.
- Added private temporary-file lifecycle management and panic containment.
- Added Rust unit tests, strict clippy checks, and ABI smoke tests.
- Added a secondary GTK3 Rust backend and common widgetset build wrapper.

### Changed

- Made Qt5 the primary supported target after successful manual testing with
  the official Qt5 Double Commander package.
- Moved build outputs to the ignored `build/` directory for release upload.
- Removed the v0.1.0 Pascal/Lazarus implementation from the current tree; it
  remains available from tag `v0.1.0` and commit `62e4ac2`.
- Replaced the dynamic MuPDF library adapter with the external `mutool`
  process boundary.

### Removed

- Removed the Pascal/Lazarus sources and local WLX SDK declarations.
- Removed the v0.1.0 Qt6 and GTK2 implementations; those Rust targets are not
  yet implemented.

## [v0.1.0] - 2026-09-22

### Added

- Rebuilt the copied Markdown WLX skeleton as a PDF F3 viewer.
- Dynamic MuPDF rendering through an exception-safe C adapter.
- Native Qt5, Qt6, GTK2, and GTK3 page viewers.
- Bounded 120 DPI rendering with a 128-page safety limit.
- Ubuntu 22.04 build instructions and third-party licensing notes.
- ABI smoke tests for PDF detection and exported metadata.

### Fixed

- Avoided static MuPDF linkage by loading a system MuPDF ABI at runtime.
