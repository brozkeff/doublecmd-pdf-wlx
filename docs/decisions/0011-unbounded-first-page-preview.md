---
status: Accepted
date: 2026-09-23
---
# Unbounded First-Page Preview

## Context and Problem Statement

The viewer renders only page 1, but the previous 128-page validation ceiling
rejected otherwise valid long PDFs without reducing preview work. The page
count remains useful for rejecting empty documents and displaying `Page 1 of
N`.

## Decision Outcome

Retain page-count validation and reject invalid, encrypted, or empty
documents, but do not impose an upper page-count limit while only the first
page is rendered. This supersedes the 128-page restriction in ADR 0004 and the
mutool fallback decision in ADR 0008. Revisit the limit if the viewer begins
rendering or preloading additional pages.
