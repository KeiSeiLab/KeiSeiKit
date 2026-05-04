---
title: pk.rs
path: kei-entity-store/src/verbs/pk.rs
dna_hash: sha256:600f2625a9920ee3
language: rust
size_loc: 71
generated: by-keidocs
---

# kei-entity-store/src/verbs/pk.rs

Shared primary-key extraction helper — bridges IntegerPk / TextPk
schemas so each verb can accept `{"id": <int>}` or `{"id": "<str>"}`
without duplicating the dispatch logic.

## Public API

- A primary-key value bound for SQLite. Text PKs carry the raw string;
- `pub fn as_string` — String form — used to render `NotFound` errors uniformly.
- `pub fn extract` — Extract the primary-key value from a verb input JSON object. `verb`
- `pub fn pk_name` — The PK column name.

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate, rusqlite, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
