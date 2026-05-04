---
title: schema.rs
path: kei-scheduler/src/schema.rs
dna_hash: sha256:9602566af1697854
language: rust
size_loc: 52
generated: by-keidocs
---

# kei-scheduler/src/schema.rs

kei-scheduler EntitySchema — declarative spec consumed by
`kei_entity_store::Store`.

Schema matches the sibling pattern (kei-task, kei-chat-store): one
primary table with standard CRUD fields plus scheduler-specific
trigger + run-tracking columns. The `name` UNIQUE constraint rides
the engine's `custom_migrations` slot because `FieldDef` doesn't
expose a UNIQUE flag — a unique index on the column provides the
same semantics.

## Public API

- Full schema list passed to the engine when opening a Store. Declared

## Related

- parent: `kei-scheduler/Cargo.toml`
- imports: kei_entity_store

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
