---
title: schema.rs
path: kei-memory-sqlite/src/schema.rs
dna_hash: sha256:e159e817c172f7f5
language: rust
size_loc: 99
generated: by-keidocs
---

# kei-memory-sqlite/src/schema.rs

SQL schema for the kei-memory-sqlite `MemoryBackend`.

Constructor Pattern: schema only, no business logic.

Single table `memory_items` keyed by DNA (PRIMARY KEY). Tags are kept
in a single TEXT column as a comma-bordered CSV (`,t1,t2,`) so an
exact-token `LIKE '%,<tag>,%'` filter does not match prefixes.

Indexes:
- `idx_memory_items_kind_key`   — supports kind + key-prefix queries.
- `idx_memory_items_created_at` — supports `since_ms` filter and
`compact(since_ms)` deletion ordering.

## Public API

- `pub const DDL` — DDL applied by [`apply_schema`]. Idempotent (`IF NOT EXISTS` everywhere).
- `pub fn apply_schema` — Apply the full schema. Idempotent — safe to call on every connection
- `pub fn encode_tags` — Encode tag list as `,t1,t2,…,` so exact-token `LIKE '%,<tag>,%'`
- `pub fn decode_tags` — Inverse of [`encode_tags`]. Robust to empty input (returns empty Vec).

## Related

- parent: `kei-memory-sqlite/Cargo.toml`
- imports: rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
