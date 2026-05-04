---
title: backend.rs
path: kei-memory-sled/src/backend.rs
dna_hash: sha256:c144e89c32533b54
language: rust
size_loc: 116
generated: by-keidocs
---

# kei-memory-sled/src/backend.rs

`SledBackend` — async `MemoryBackend` impl that wraps the sync
`SledStore` via `tokio::task::spawn_blocking`.

## Public API

- `pub fn from_path` — Open a sled DB at `path` and stamp this backend with a fresh DNA.
- `pub fn inner_store` — Borrow the inner store (mostly for tests / advanced callers).
- Apply post-scan filters (key_prefix, tag_any, since_ms, limit).

## Related

- parent: `kei-memory-sled/Cargo.toml`
- imports: crate, kei_runtime_core, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
