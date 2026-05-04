---
title: store.rs
path: kei-scheduler/src/store.rs
dna_hash: sha256:413a3b6c4fe6fc85
language: rust
size_loc: 32
generated: by-keidocs
---

# kei-scheduler/src/store.rs

Scheduler store — thin shim over `kei_entity_store::Store`.

Mirrors the kei-chat-store pattern: the engine owns DDL + migration
transactions, and this crate adds scheduler-specific SQL helpers
(`schedule`, `cancel`, `list_due`, `mark_run`) that live in sibling
modules.

## Related

- parent: `kei-scheduler/Cargo.toml`
- imports: anyhow, crate, kei_entity_store, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
