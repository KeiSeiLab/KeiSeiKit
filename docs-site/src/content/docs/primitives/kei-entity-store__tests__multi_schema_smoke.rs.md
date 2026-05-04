---
title: multi_schema_smoke.rs
path: kei-entity-store/tests/multi_schema_smoke.rs
dna_hash: sha256:741e2484cc5c69fa
language: rust
size_loc: 148
generated: by-keidocs
---

# kei-entity-store/tests/multi_schema_smoke.rs

Multi-schema smoke tests — verify that `Store::open` accepts a
slice of `&EntitySchema`, runs every schema's migrations inside a
single transaction, and that verbs dispatched per-schema work
independently against the same underlying connection.

Added 2026-04-23 with the multi-schema breaking change. Parity
target: unblock kei-chat-store from its single-schema constraint
(two entity types — integer-PK messages + text-PK sessions).

## Public API

- Schema that deliberately breaks on migration (custom_migrations

## Related

- parent: `kei-entity-store/tests`
- imports: kei_entity_store, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
