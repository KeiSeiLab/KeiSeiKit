---
title: schema.rs
path: kei-artifact/src/schema.rs
dna_hash: sha256:907bf0030be9f738
language: rust
size_loc: 50
generated: by-keidocs
---

# kei-artifact/src/schema.rs

SQL schema DDL + migrations for the artifact store.

Two tables:
- `schemas`   — registered JSON Schemas by name (SSoT for validation).
- `artifacts` — typed content + metadata + parent pointer for handoff chain.

## Public API

- `pub const MIGRATIONS` — Ordered migrations. Index = schema version. Append only; never reorder.
- `pub fn migrate` — Apply pending migrations. Uses pragma `user_version` as the version cursor.
- `pub const KNOWN_SCHEMAS` — Canonical list of artifact schema names shipped with this primitive.

## Related

- parent: `kei-artifact/Cargo.toml`
- imports: rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
