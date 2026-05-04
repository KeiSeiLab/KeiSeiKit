---
title: backend.rs
path: kei-memory-postgres/src/backend.rs
dna_hash: sha256:ea7ced4c8c134e84
language: rust
size_loc: 170
generated: by-keidocs
---

# kei-memory-postgres/src/backend.rs

`MemoryBackend` impl over `PgStore`. One backend = one DNA. Many
backends can share the same `Arc<PgStore>`.

## Public API

- `pub fn new` — Build with a fresh DNA. `body` defaults to `b"pg-v16"` to

## Related

- parent: `kei-memory-postgres/Cargo.toml`
- imports: crate, kei-sleep-sync.sh, kei_runtime_core, std, tokio_postgres

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
