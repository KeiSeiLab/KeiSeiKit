---
title: real_text_pk_smoke.rs
path: kei-entity-store/tests/real_text_pk_smoke.rs
dna_hash: sha256:1601ef3923202d26
language: rust
size_loc: 267
generated: by-keidocs
---

# kei-entity-store/tests/real_text_pk_smoke.rs

Smoke tests for the four M1 / M4 / M5 engine improvements:

1. `FieldKind::TextPk` — TEXT primary key schemas with caller-
supplied UUID-style ids.
2. `FieldKind::Real` / `RealDefault` — REAL columns round-tripped as
f64 through create + get.
3. `FieldKind::TextArchiveEnum` — archive verb writes the archived
sentinel string on schemas that encode status as a TEXT enum.
4. `EdgeKeyKind::TextPairWithMetadata` — text-keyed edges with
optional weight / id / created_at columns, used by rank.

## Related

- parent: `kei-entity-store/tests`
- imports: kei_entity_store, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
