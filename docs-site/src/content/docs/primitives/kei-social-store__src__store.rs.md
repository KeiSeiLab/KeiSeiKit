---
title: store.rs
path: kei-social-store/src/store.rs
dna_hash: sha256:d8f83574d8562946
language: rust
size_loc: 31
generated: by-keidocs
---

# kei-social-store/src/store.rs

Social store — thin shim over `kei_entity_store::Store`.

Layer-A convergence (2026-04-23): generic CRUD verbs on `people`
(create/get/search/list) run through `kei_entity_store::verbs::*`
with `SOCIAL_SCHEMA`. Organization and interaction helpers still
use the raw connection against tables declared in
`custom_migrations` — they are not generic-CRUD.

## Related

- parent: `kei-social-store/Cargo.toml`
- imports: anyhow, crate, kei_entity_store, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
