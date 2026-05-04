---
title: store.rs
path: kei-memory-sqlite/src/store.rs
dna_hash: sha256:7c4d0c5aa237c6e6
language: rust
size_loc: 95
generated: by-keidocs
---

# kei-memory-sqlite/src/store.rs

SQLite-backed storage layer. The async surface lives in `backend.rs`;
this module is sync (rusqlite is sync) and exposes a `SqliteStore`
whose Arc-cloned handle is shared by the backend, which wraps the
actual blocking calls in `tokio::task::spawn_blocking`.

Connection is guarded by `std::sync::Mutex` because rusqlite's
`Connection` is not `Sync` on its own. The blocking surface is small
(one `lock()` per backend op) and the spawn_blocking thread holds it
only for the duration of the SQL.

## Public API

- `pub struct SqliteStore` — Owned SQLite handle. Cheap to wrap in `Arc` for sharing across
- `pub fn from_path` — Open or create a SQLite DB at `path`. Schema is applied
- `pub fn from_memory` — In-memory store for tests / ephemeral fixtures. Schema applied.
- `pub fn lock` — Borrow the connection mutex. Backend uses this from inside

## Related

- parent: `kei-memory-sqlite/Cargo.toml`
- imports: crate, rusqlite, std

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
