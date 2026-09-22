# Changelog

## [v0.1.0] - 2026-09-22

### Added

- Rebuilt the copied Markdown WLX skeleton as a PDF F3 viewer.
- Dynamic MuPDF rendering through an exception-safe C adapter.
- Native Qt5, Qt6, GTK2, and GTK3 page viewers.
- Bounded 120 DPI rendering with a 128-page safety limit.
- Ubuntu 22.04 build instructions, exact tested package versions, and complete third-party licensing notes.
- ABI smoke tests for PDF detection and exported metadata.

### Fixed

- Avoid static MuPDF linkage and its transitive ELF dependency closure; load a system MuPDF ABI at runtime instead.
