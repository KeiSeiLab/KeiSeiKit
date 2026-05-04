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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
