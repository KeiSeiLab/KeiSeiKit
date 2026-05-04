---
title: lib.rs
path: kei-memory-sqlite/src/lib.rs
dna_hash: sha256:a3f9568c9d456c4b
language: rust
size_loc: 32
generated: by-keidocs
---

# kei-memory-sqlite/src/lib.rs

kei-memory-sqlite — `MemoryBackend` impl over SQLite (rusqlite bundled).

Hosted Sleep Wave 6 atomar. Offline-first, single-process, embedded
storage. Suitable for:
- per-user VM local memory store (file-backed)
- offline-first agents needing structured `MemoryItem` storage with
indexed query (kind / key-prefix / tags / time)
- test fixtures (in-memory via `SqliteStore::from_memory`)

Constructor Pattern (one file = one responsibility):
- [`error`]   : crate-local error type, mappable into `kei_runtime_core::Error`.
- [`schema`]  : DDL + idempotent `apply_schema`.
- [`store`]   : low-level rusqlite handle + path/in-memory constructors.
- [`backend`] : [`backend::SqliteBackend`] glues `SqliteStore` to the
`MemoryBackend` trait + carries a DNA.

Out of scope:
- cross-process concurrency beyond what SQLite offers (use Redis/sled siblings)
- remote mirroring (`mirror_to_remote` returns `Provider` error;
git-push is the responsibility of `kei-sleep-sync.sh` per RULE 0.15)

## Related

- parent: `kei-memory-sqlite/Cargo.toml`

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
