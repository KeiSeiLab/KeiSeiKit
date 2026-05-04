---
title: archive_smoke.rs
path: kei-entity-store/tests/archive_smoke.rs
dna_hash: sha256:97f84c398b045638
language: rust
size_loc: 99
generated: by-keidocs
---

# kei-entity-store/tests/archive_smoke.rs

Archive-verb smoke tests.

Covers kei-chat-store migration: schemas opt-in to soft-delete via
`archived_field: Some("archived")`. The verb flips the column + an
optional `<field>_at` sibling timestamp.

## Related

- parent: `kei-entity-store/tests`
- imports: kei_entity_store, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
