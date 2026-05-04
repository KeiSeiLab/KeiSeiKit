---
title: schema.rs
path: kei-prune/src/schema.rs
dna_hash: sha256:46181465dc8d4ee2
language: rust
size_loc: 36
generated: by-keidocs
---

# kei-prune/src/schema.rs

Schema for the pruning sidecar.

Constructor Pattern: one cube = sidecar DDL + idempotent installer.
We deliberately do NOT touch the `agents` table CHECK constraint —
that is owned by kei-ledger and cannot be extended from outside
without risking a migration desync. Instead we keep a lightweight
companion table keyed on the same `agent_id`.

## Public API

- `pub const SIDECAR_DDL` — DDL for the `prune_retirements` sidecar.
- `pub fn ensure_schema` — Create the sidecar table if it does not yet exist.

## Related

- parent: `kei-prune/Cargo.toml`
- imports: crate, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
