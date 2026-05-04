---
title: schema.rs
path: kei-memory/src/schema.rs
dna_hash: sha256:d2cec70682d4c2b7
language: rust
size_loc: 95
generated: by-keidocs
---

# kei-memory/src/schema.rs

SQL schema for the kei-memory offline analyzer.

Constructor Pattern: schema + migration runner, no business logic.
DB default path: `~/.claude/memory/kei-memory.sqlite`.
Any structural change MUST append a new migration; never edit history.

## Public API

- `pub const MIGRATIONS` — Ordered migrations. Index = schema version. Never reorder.
- `pub fn migrate` — Apply all pending migrations. Stores version in `PRAGMA user_version`.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
