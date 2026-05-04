---
title: store.rs
path: kei-sage/src/store.rs
dna_hash: sha256:530b6de3da4934a9
language: rust
size_loc: 114
generated: by-keidocs
---

# kei-sage/src/store.rs

Knowledge-unit CRUD + FTS indexer.

`Store::open` / `Store::open_memory` delegate to
`kei_entity_store::Store` which runs `SAGE_SCHEMA` migrations.
The sage-specific `add_unit` / `update_unit` / `delete_unit`
helpers stay here because they use `INSERT OR REPLACE` idempotency
by `vault_path` and maintain sage's custom FTS table (`fts_knowledge`
with column `unit_id`) — engine's generic `create` verb assumes a
different FTS shape (`fts_<table>` with column `<table>_id`).

## Public API

- `pub fn add_unit` — Insert a new knowledge unit. Indexes title+content into FTS5. Idempotent by vault_path.

## Related

- parent: `kei-sage/Cargo.toml`
- imports: anyhow, chrono, crate, kei_entity_store, rusqlite, std

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
