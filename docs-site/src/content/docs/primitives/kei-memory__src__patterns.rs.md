---
title: patterns.rs
path: kei-memory/src/patterns.rs
dna_hash: sha256:91db88b1948ba23c
language: rust
size_loc: 101
generated: by-keidocs
---

# kei-memory/src/patterns.rs

Pattern detector — recurring event-classes.

Constructor Pattern: one cube, one read/write responsibility.
A "pattern" is an event_class that occurred ≥2 times in ONE session
(in-session recurrence) or ≥2 times across DIFFERENT sessions
(cross-session recurrence). Results are persisted into `patterns` and
also returned to the caller for display.

## Public API

- `pub fn detect_in_session` — Detect in-session recurrences for `session_id`. Persists rows.
- `pub fn detect_cross_session` — Detect cross-session recurrences. Does NOT persist (history aggregate).
- List all patterns in the persistent table (newest first).

## Related

- parent: `kei-memory/Cargo.toml`
- imports: rusqlite

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
