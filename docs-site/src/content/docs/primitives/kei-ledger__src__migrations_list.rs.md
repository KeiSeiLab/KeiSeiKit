---
title: migrations_list.rs
path: kei-ledger/src/migrations_list.rs
dna_hash: sha256:8ea125901f99b9d1
language: rust
size_loc: 111
generated: by-keidocs
---

# kei-ledger/src/migrations_list.rs

Ordered migration DDL list — single source of truth for `agents`
table evolution + the v8 `skill_invocations` companion table.

Constructor Pattern: extracted from `schema.rs` so the runner cube
stays under the 200-LOC ceiling. NEVER reorder; append only. Index =
schema version (1-based via `MIGRATIONS[i]`, target = i+1).

## Public API

- `pub const MIGRATIONS` — Ordered migrations. Index = schema version (1-based; target = i+1).

## Related

- parent: `kei-ledger/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
