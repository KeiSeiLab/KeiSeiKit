---
title: backend.rs
path: kei-memory-sqlite/src/backend.rs
dna_hash: sha256:cf99474c168e2c12
language: rust
size_loc: 191
generated: by-keidocs
---

# kei-memory-sqlite/src/backend.rs

[`SqliteBackend`] — `MemoryBackend` impl over [`SqliteStore`].

All SQL ops are synchronous (rusqlite) and wrapped in
`tokio::task::spawn_blocking` so the async runtime is never stalled.
The store itself is shared via `Arc`; cloning a backend is cheap.

## Public API

- `pub struct SqliteBackend` — SQLite-backed [`MemoryBackend`]. Holds its own DNA + an `Arc<SqliteStore>`.
- `pub fn new` — Construct from an already-built store + DNA. Parent DNA optional.
- `pub fn inner_store` — Borrow the underlying store (e.g. for sibling backends to share it).
- Compose dynamic SELECT with parameter list. Order DESC by created_at_ms.
- Append WHERE-clause filters in stable order. Splits to keep
- Map one row → `MemoryItem`.

## Related

- parent: `kei-memory-sqlite/Cargo.toml`
- imports: async_trait, crate, kei_runtime_core, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
