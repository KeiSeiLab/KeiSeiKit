---
title: store.rs
path: kei-task/src/store.rs
dna_hash: sha256:6b7bd53b11a06395
language: rust
size_loc: 101
generated: by-keidocs
---

# kei-task/src/store.rs

Task store — thin shim over `kei_entity_store::Store`.

Layer-A convergence pilot (2026-04-23): generic CRUD (create / get /
update) now runs through `kei_entity_store::verbs::*` using the
declarative `TASK_SCHEMA`. Public surface is preserved byte-for-byte
so existing integration tests and callers (`atoms::create`,
`milestones`, `deps`, `search`) compile unchanged.

## Related

- parent: `kei-task/Cargo.toml`
- imports: anyhow, crate, kei_entity_store, rusqlite, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
