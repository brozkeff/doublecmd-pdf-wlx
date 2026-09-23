# Development Plans

## Goal

Make F3 preview useful immediately for multipage PDFs, then investigate
non-blocking page navigation. Version 0.3.0 keeps the fast first-page preview
and adopts Poppler Splash after manual quality and CMYK comparison.

## v0.2.3: Unbounded first-page preview

- [x] Remove the page-count ceiling while retaining page-count validation.
- [x] Keep rendering only page 1 at 120 DPI and report the document page count.
- [x] Update the package version, exported version string, and user docs.

## v0.2.2: Qt5-only maintenance patch

- [x] Remove the unsupported GTK3 Rust crate and its build and smoke-test paths.
- [x] Close the active Lister window when Escape is pressed, retaining the
  host's existing Q shortcut.
- [x] Move Rust unit test bodies into separate files under `tests/unit/`.

## v0.2.1: First-page fast path

- [x] Render and display only page 1 in the Qt5 backend.
- [x] Keep page-count validation and 120 DPI rendering. The 128-page ceiling
      was removed in v0.2.3 because only the first page is rendered.
- [x] Show `Page 1 of N` when the PDF contains more than one page.
- [x] Update the Qt5 package version, exported version string, and user docs.
- [ ] Benchmark 1-page, 30-page, and 128-page fixtures for time to first page,
  total render time, peak memory, and temporary disk use.
- [ ] Confirm the 30-page fixture reaches its first visible page within 20% of
  the 1-page fixture on the same machine.
- [x] Manually verify the Qt5 preview in the official Double Commander package.

Both backends reject invalid, encrypted, and empty documents without an upper
page-count limit. The `mutool` backend's preview images remain private
temporary PNG files and are removed on normal success and failure paths; a
process crash can still leave a file. Poppler holds its rendered page in memory.

## v0.3.0: Poppler Splash default backend

### Experimental Poppler Splash branch

- [x] Compare Poppler QPainter and Splash against `mutool` and Poppler Cairo
      using the private CMYK PDF under ignored `tmp/`.
- [x] Keep `mutool` as an optional compile-time backend and make Poppler Splash
      the default renderer.
- [x] Render the first Poppler page to an in-memory image without preview files.
- [x] Enable Poppler Splash graphics/text antialiasing and light text hinting.
- [x] Compare plugin `ListLoad` latency on the same private CMYK PDF. With the
      Splash hints enabled, the five-run medians were 209 ms for Poppler and
      299 ms for `mutool`; earlier runs measured 201 ms and 346 ms respectively.
- [ ] Compare process-tree peak memory. A first harness run measured 48.7 MB
      for Poppler and 44.0 MB for `mutool`; this is only one sample and does not
      capture the child's memory as a combined total.
- [x] Check the runtime dependency closure: the Splash build links to
      `libpoppler-qt5` and loads Poppler core, LCMS, font, and other libraries.
- [x] Confirm both backends load a generated 129-page PDF and render only page 1.
- [x] Manually load both backend builds in Double Commander. After enabling
      Splash antialiasing and light hinting, the user confirmed rendering quality
      is excellent; Poppler also gives good CMYK colors on the private fixture.
- [x] Check EUPL compatibility against the official matrix: it marks dynamic
      linking to GPLv2 and GPLv3 as compatible with an EUPL-distributed larger
      work; keep Poppler's own notices and verify exact distro package terms.
- [x] Confirm the Splash antialiasing and hinting settings in Double Commander.
- [x] Record backend-specific artifact licenses and retain EUPL as a source
      license option; include selected license sidecars in release builds.

## Future: on-demand rendering

- [ ] Render the first page before doing background work; then render the
  visible page and a small look-ahead window asynchronously.
- [ ] Cancel child work and discard queued pages when the WLX window closes or
  another file is opened. Bound concurrency, memory, and retained page images.
- [ ] Load rendered page pixels in memory so process termination cannot leave
  preview files behind.
- [ ] Compare the existing `mutool` path with a pure-Rust renderer such as
  Hayro, checking latency, PDF feature coverage, dependencies,
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

Keep first-page rendering until a real user workflow calls for more pages.
Treat process-tree memory measurement and on-demand page rendering as follow-up
work; the release renderer and selected artifact licenses are now documented.
