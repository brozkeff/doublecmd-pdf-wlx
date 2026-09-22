# Development Plans

## Goal

Make F3 preview useful immediately for multipage PDFs, then investigate a
profile-aware renderer and non-blocking page navigation. The Qt5-only v0.2.2
preview renders only the first page so opening a long document does not
rasterize pages the user cannot see yet.

## v0.2.2: Qt5-only maintenance patch

- [x] Remove the unsupported GTK3 Rust crate and its build and smoke-test paths.
- [x] Close the active Lister window when Escape is pressed, retaining the
  host's existing Q shortcut.
- [x] Move Rust unit test bodies into separate files under `tests/unit/`.

## v0.2.1: First-page fast path

- [x] Render and display only page 1 in the Qt5 backend.
- [x] Keep page-count validation, the 128-page limit, and 120 DPI rendering.
- [x] Show `Page 1 of N` when the PDF contains more than one page.
- [x] Update the Qt5 package version, exported version string, and user docs.
- [ ] Benchmark 1-page, 30-page, and 128-page fixtures for time to first page,
  total render time, peak memory, and temporary disk use.
- [ ] Confirm the 30-page fixture reaches its first visible page within 20% of
  the 1-page fixture on the same machine.
- [x] Manually verify the Qt5 preview in the official Double Commander package.

The page-count check still rejects invalid, encrypted, empty, and over-limit
documents. Preview images remain private temporary PNG files and are removed
on normal success and failure paths. A process crash can still leave a file.

## v0.3.0: Profile-aware, on-demand rendering

- [ ] Prototype a renderer path that can assign fallback source profiles while
  preserving profiles and output intents embedded in the PDF.
- [ ] Use `assets/icc/eciCMYK_v2.icc` for untagged CMYK and
  `assets/icc/sRGB.icm` for untagged RGB, then convert to screen RGB. Validate
  the assumption with representative PDFs before treating it as universal.
- [x] Stage the fresh path-free ArgyllCMS CMYK-to-sRGB link generated at low
      `-ql` quality with a 6-point CLUT.
- [ ] Compare its color output with a freshly generated device link and
  representative PDFs before using it in the viewer.
- [ ] Render the first page before doing background work; then render the
  visible page and a small look-ahead window asynchronously.
- [ ] Cancel child work and discard queued pages when the WLX window closes or
  another file is opened. Bound concurrency, memory, and retained page images.
- [ ] Load rendered page pixels in memory so process termination cannot leave
  preview files behind.
- [ ] Compare the existing `mutool` path with a pure-Rust renderer such as
  Hayro, checking latency, PDF feature coverage, profile support, dependencies,
  and licensing before changing the backend.

## Vector viewer investigation

- [ ] Check Qt PDF availability in the supported Qt5 Double Commander
  distributions and compare Poppler Qt5 if Qt PDF is unavailable.
- [ ] Prototype vector page display inside the host's existing GUI loop, with
  pages loaded as they enter the viewport rather than preloading the document.
- [ ] Compare startup latency, zoom quality, navigation, memory, dependencies,
  licensing, and host ABI integration against asynchronous raster rendering.
- [ ] Capture any selected renderer or viewer architecture in a new ADR before
  implementation.

Keep the v0.2.1 first-page path unless measurements or a real user workflow
show that more pages should be requested. A vector or Rust renderer should be
adopted only if it meets the latency goal and handles the representative PDF
fixtures reliably.
