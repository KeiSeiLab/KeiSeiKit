---
title: comments_routes_init.rs
path: kei-cortex/src/comments_routes_init.rs
dna_hash: sha256:c0cdb8a7b2788dd3
language: rust
size_loc: 95
generated: by-keidocs
---

# kei-cortex/src/comments_routes_init.rs

Init / store-handle / validators for `comments_routes`.

Constructor Pattern: `comments_routes.rs` is the router + handlers,
this file owns the side-effecting bootstrap (open DB, migrate) and
the pure input validators. Splitting keeps each file <200 LOC.

## Public API

- `pub type SharedStore` — Shared, thread-safe handle injected via Extension into every handler.
- `pub fn global_store` — Process-wide store handle. Opened lazily; on first-use failure we

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, kei_comments, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
