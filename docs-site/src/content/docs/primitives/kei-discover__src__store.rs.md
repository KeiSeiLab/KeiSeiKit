---
title: store.rs
path: kei-discover/src/store.rs
dna_hash: sha256:243747d5205ae360
language: rust
size_loc: 35
generated: by-keidocs
---

# kei-discover/src/store.rs

`Store` — thin shim over `kei_entity_store::Store` wired to
`DISCOVER_SCHEMA`.

Two constructors: `open(path)` for on-disk and `open_memory()` for
unit tests. The inner connection is exposed read-only via `.conn()`
so the per-verb modules can call `kei_entity_store::verbs::*`
directly without taking a mutable borrow on the Store.

## Related

- parent: `kei-discover/Cargo.toml`
- imports: crate, kei_entity_store, rusqlite, std

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
