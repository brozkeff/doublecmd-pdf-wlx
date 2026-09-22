# Development Plans

## Goal

Make F3 preview useful immediately for multipage PDFs. Version 0.2.0 renders
every page synchronously before returning from `ListLoad`; a 30-page document
therefore performs unnecessary work when the user only needs a quick preview.
Rasterizing the complete document also uses more memory than a conventional
vector PDF viewer.

## v0.2.1: First-page fast path

- [ ] Add benchmark fixtures for 1-page, 30-page, and 128-page PDFs.
- [ ] Record time to first visible page, total render time, peak memory, and
  temporary disk usage for the Qt5 backend.
- [ ] Change the default preview to render and display only page 1.
- [ ] Keep the page-count check so invalid, encrypted, empty, and over-limit
  documents still fail safely.
- [ ] Show a small `Page 1 of N` indicator when the document has more pages.
- [ ] Add tests proving that the fast path invokes `mutool draw` only for page
  1 and removes all temporary files after loading.
- [ ] Repeat the manual F3 integration test in official Qt5 Double Commander.
- [ ] Document the first-page behavior and provide an explicit way to request
  the complete document only if a real user workflow requires it.

Acceptance target: opening the 30-page fixture should load only one raster and
reach the first visible page within 20% of the 1-page fixture on the same
machine.

## v0.3.0: Progressive multipage preview

- [ ] Prototype page-at-a-time rendering after the first page is visible.
- [ ] Render only the current page and a small look-ahead window instead of
  rasterizing the full document.
- [ ] Run `mutool` outside the GUI thread and marshal completed images back to
  Qt safely.
- [ ] Cancel child processes and discard queued pages when the WLX window
  closes or Double Commander opens another file.
- [ ] Bound concurrent renders, memory, temporary disk usage, and retained
  pixmaps.
- [ ] Add visible loading and failure states without blocking page 1.
- [ ] Decide whether GTK3 should share the progressive renderer or remain a
  build-only secondary backend.

Do not add asynchronous complexity until the first-page fast path has been
measured. For a quick-look plugin, first-page-only behavior may be the complete
solution.

## Vector rendering investigation

- [ ] Evaluate Qt PDF (`QPdfDocument` and `QPdfView`) availability across the
  supported Qt5 Double Commander distributions.
- [ ] Evaluate Poppler Qt5 as a fallback only if Qt PDF is not practical.
- [ ] Compare dependency size, startup latency, zoom quality, navigation,
  licensing, and ABI compatibility against the `mutool` raster pipeline.
- [ ] Verify that a candidate viewer can be parented safely into the WLX host
  without starting another Qt application or event loop.
- [ ] Capture the chosen vector or raster architecture in a new ADR before
  implementation.

A vector backend should replace the raster pipeline only when it builds on the
target distributions, improves actual preview latency or interaction, and does
not introduce disproportionate packaging or licensing costs.
