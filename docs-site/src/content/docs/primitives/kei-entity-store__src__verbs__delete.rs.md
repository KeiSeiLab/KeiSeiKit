---
title: delete.rs
path: kei-entity-store/src/verbs/delete.rs
dna_hash: sha256:624b42a489c6b8d8
language: rust
size_loc: 86
generated: by-keidocs
---

# kei-entity-store/src/verbs/delete.rs

`delete` verb — hard DELETE by id, OR soft (if schema has an
`archived` integer field, flips it to 1).

The hard-delete path wraps the `fts_<table>` DELETE and the base-table
DELETE in a single `unchecked_transaction`, so a mid-flight FTS failure
rolls back the row delete too (mirrors create/update C2 fix).

## Public API

- FTS DELETE + base-table DELETE in one transaction. If either fails,

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate, rusqlite, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
