---
title: error.rs
path: kei-scheduler/src/error.rs
dna_hash: sha256:b2d4b83f5475e374
language: rust
size_loc: 51
generated: by-keidocs
---

# kei-scheduler/src/error.rs

Typed errors for kei-scheduler. `Error` is the public wrapper;
`ParseError` surfaces trigger-spec parse failures separately so
callers (and tests) can discriminate without string-matching.

## Public API

- Trigger-spec parse failures. Pure function — no DB contact.
- Public scheduler error. Wraps rusqlite + anyhow + ParseError.
- `pub fn from_insert` — Inspect a rusqlite error and reclassify `UNIQUE constraint

## Related

- parent: `kei-scheduler/Cargo.toml`
- imports: thiserror

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
