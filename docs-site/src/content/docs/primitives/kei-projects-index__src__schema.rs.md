---
title: schema.rs
path: kei-projects-index/src/schema.rs
dna_hash: sha256:b792a273485c380c
language: rust
size_loc: 41
generated: by-keidocs
---

# kei-projects-index/src/schema.rs

SQLite schema for kei-projects-index.

Constructor Pattern: one cube = schema DDL + initialiser. No business
logic. Single source of truth for the `projects` table consumed by
both `kei-projects-watcher` (writer) and `kei-cortex` (reader).

## Public API

- `pub const SCHEMA_DDL` — Full schema applied by `init`. Idempotent (`IF NOT EXISTS` everywhere).
- `pub fn init` — Apply (or re-apply) the schema. Idempotent — safe to call on every

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
