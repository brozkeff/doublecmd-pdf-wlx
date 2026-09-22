---
status: Superseded: by 0007-dynamic-mupdf-binding.md
date: 2026-09-22
---
# Dynamic MuPDF Release Option

## Context and Problem Statement

Static MuPDF linking makes the plugin self-contained but makes every binary a combined AGPL work with a larger third-party dependency closure. GitHub release assets would be easier to distribute if the plugin loaded the system MuPDF library at runtime and shipped no MuPDF object code.

## Decision Outcome

Treat dynamic `dlopen`/`dlsym` binding as a packaging improvement, not an automatic license exemption. Before adopting it, verify the target distribution's MuPDF soname and test missing-library fallback. If adopted, publish the plugin as an EUPL source work with explicit AGPL dependency notices, system-package instructions, and no bundled MuPDF; obtain legal review or an Artifex commercial license if stronger separation is required.
