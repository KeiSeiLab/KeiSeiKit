---
title: store.rs
path: kei-crossdomain/src/store.rs
dna_hash: sha256:2ec6b77500acbb7c
language: rust
size_loc: 38
generated: by-keidocs
---

# kei-crossdomain/src/store.rs

Crossdomain store — thin shim over `kei_entity_store::Store`.

Layer-A convergence (2026-04-23): connection lifecycle + migrations +
`PRAGMA user_version` stamping now ride the shared engine via
`CROSSDOMAIN_SCHEMA`. Public surface preserved byte-for-byte so
`edges.rs`, `bfs.rs`, `auto_link.rs`, and integration tests compile
unchanged.

Generic CRUD verbs are NOT wired here — kei-crossdomain is an
edges-only store with bespoke TextPair+extras columns; see
`schema.rs` for the architectural note on why engine's `link`/`rank`
verbs cannot serve this crate without a destructive schema rewrite.

## Related

- parent: `kei-crossdomain/Cargo.toml`
- imports: anyhow, crate, kei_entity_store, rusqlite, std

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
