---
title: coaccess.rs
path: kei-memory/src/coaccess.rs
dna_hash: sha256:9f708d2b5f84e431
language: rust
size_loc: 83
generated: by-keidocs
---

# kei-memory/src/coaccess.rs

Co-access tracking — files touched within a 5-minute window.

Constructor Pattern: one cube, single responsibility.
Derived from an in-house implementation, algorithmic spec documented in coaccess.md.

Session_id IS used to scope the window query (avoiding cross-session
false co-access — we never pair file_a from session X with file_b
from session Y), but it isn't part of the coaccess row primary key
(the PK is the canonical file pair). This means a file pair seen in
5 sessions has 1 row, not 5 — counts aggregate across sessions so
cross-session recurrences surface in `patterns`.

## Public API

- `pub fn record_coaccess` — Insert (or increment) pair entries for the new file vs any other file
- Return (file_a, file_b, count) triples ordered by co-access count DESC.

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
