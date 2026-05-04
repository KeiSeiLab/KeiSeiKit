---
title: error.rs
path: kei-memory/src/error.rs
dna_hash: sha256:018c1207c1161ccc
language: rust
size_loc: 27
generated: by-keidocs
---

# kei-memory/src/error.rs

Error type for kei-memory.

Constructor Pattern: this cube only declares the error enum + Result alias.
Wave A motive — `ingest.rs:55-56` was abusing
`rusqlite::Error::InvalidParameterName` to wrap an `io::Error`. That hides
the real failure source from callers and confuses operators reading logs.
`KeiMemoryError` separates the four failure domains we actually have.

## Public API

- `pub type Result` — Crate-wide Result alias for paths that mix IO + parse + DB.

## Related

- parent: `kei-memory/Cargo.toml`
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
