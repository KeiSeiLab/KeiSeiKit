---
title: update.rs
path: kei-entity-store/src/verbs/update.rs
dna_hash: sha256:18b5a471b7c061b0
language: rust
size_loc: 199
generated: by-keidocs
---

# kei-entity-store/src/verbs/update.rs

`update` verb — partial update by id. Only declared schema keys
that appear in the input JSON are written. Type mismatch →
`InvalidType` (no silent coercion). UPDATE + FTS reindex run in a
single transaction so a mid-flight failure leaves neither the row
nor the FTS entry in a torn state.

## Public API

- Rebuild the FTS5 row after the primary UPDATE. INVARIANT: FTS

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: chrono, crate, rusqlite, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
