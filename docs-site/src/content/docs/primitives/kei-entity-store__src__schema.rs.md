---
title: schema.rs
path: kei-entity-store/src/schema.rs
dna_hash: sha256:cc7acbe6e9d85027
language: rust
size_loc: 163
generated: by-keidocs
---

# kei-entity-store/src/schema.rs

EntitySchema — declarative description of one entity table.

A sibling crate (e.g. kei-task) defines a `static EntitySchema` and
passes a reference into every verb call. The engine reads this
structure to know: table name, fields to INSERT/SELECT, FTS columns,
edge table (for link/rank), and which verbs are enabled.

## Public API

- Field kinds the engine knows how to bind for INSERT / UPDATE and
- INTEGER PRIMARY KEY — exactly one PK per schema. Name = "id".
- TEXT PRIMARY KEY — caller supplies the PK value (e.g. UUID).
- INTEGER NOT NULL (with optional DEFAULT 0).
- INTEGER, default 0.
- TEXT NOT NULL (no default).
- TEXT with empty-string default.
- TEXT NOT NULL with explicit default value (held in `default`).
- TEXT NOT NULL representing a soft-delete enum with named
- REAL (f64) NOT NULL, default 0.0.
- REAL (f64) NOT NULL with an explicit default (held in
- Unix-timestamp INTEGER auto-stamped on insert (created_at).
- Unix-timestamp INTEGER auto-stamped on insert + update (updated_at).
- Edge-key storage strategy for the schema's `edge_table`.
- Extended text-pair edge with optional metadata columns and
- Name of the "from" TEXT key column. Defaults to `"src_path"`
- Name of the "to" TEXT key column. Defaults to `"dst_path"`.
- Emit `edge_id INTEGER PRIMARY KEY AUTOINCREMENT` column.
- Emit `weight REAL NOT NULL DEFAULT 1.0` column.
- Emit `created_at INTEGER NOT NULL` column auto-stamped on
- Extra typed columns appended after the standard metadata.
- `pub fn is_text` — True if this edge variant uses TEXT keys (any text variant).
- Declarative schema for one entity.
- Human-readable entity name — used in error messages.
- SQL table name for the primary entity rows.
- Column order — MUST start with the PK.
- Verb whitelist — e.g. ["create","get","search","update","delete"].
- If `Some`, engine creates an FTS5 virtual table `fts_<table>`
- If `Some`, engine creates `<edge_table>` for the `link` verb.
- Edge-table key layout. Default `IntegerPair` preserves legacy
- If `Some`, enables the `archive` verb. Names the column used as
- Arbitrary DDL statements run after the primary table + FTS +
- `pub fn pk` — Returns the PK column (integer or text). Panics if the schema
- `pub fn verb_enabled` — Returns true if `verb` appears in `enabled_verbs`.
- `pub fn writable_fields` — Returns the list of non-PK field names, in order. Used by the
- `pub fn field` — Look up a field by name.

## Related

- parent: `kei-entity-store/Cargo.toml`

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
