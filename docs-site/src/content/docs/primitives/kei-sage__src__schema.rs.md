---
title: schema.rs
path: kei-sage/src/schema.rs
dna_hash: sha256:968046887f584705
language: rust
size_loc: 113
generated: by-keidocs
---

# kei-sage/src/schema.rs

SQLite schema — declarative via `kei_entity_store::EntitySchema`.

Primary entity = `knowledge_units` ("unit"). Secondary tables (tags,
unit_tags, edges, fts_knowledge) ship as `custom_migrations` because
they pre-date the generic engine and carry sage-specific columns
(edge `id`/`weight`/`created_at`, FTS `unit_id`-named column, unique
partial index on `vault_path`).

Why `edge_table: None` + `fts_columns: None`:
- Engine's default `TextPair` edge layout lacks `id`/`weight`/
`created_at` that sage's `list_outgoing` returns.
- Engine's FTS auto-table name is `fts_<table>` with column
`<table>_id` — sage uses `fts_knowledge` with column `unit_id`.

The primary-table DDL produced by the engine matches the legacy
`knowledge_units` layout byte-for-byte (every column maps to an
engine `FieldKind`), so opening an existing sage DB stays idempotent.

## Public API

- Engine-owned primary-table fields for `knowledge_units`.
- Extra indexes on `knowledge_units` beyond the engine's per-field
- Tags tables (currently unused by the CLI but preserved for parity
- Typed wikilink edges between `vault_path`s — src_path/dst_path text
- FTS5 virtual table — legacy column name `unit_id` kept so existing
- Declarative SSoT for sage's SQLite layout. `edge_key_kind` is
- `pub fn create_schema` — Apply schema + FTS5 virtual table. Idempotent.

## Related

- parent: `kei-sage/Cargo.toml`
- imports: kei_entity_store, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
