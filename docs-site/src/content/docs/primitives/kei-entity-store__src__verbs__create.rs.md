---
title: create.rs
path: kei-entity-store/src/verbs/create.rs
dna_hash: sha256:13a1b30f111e90e1
language: rust
size_loc: 191
generated: by-keidocs
---

# kei-entity-store/src/verbs/create.rs

`create` verb — INSERT one row using fields declared on the schema.
Per-kind value defaulting lives in `create_defaults`.

TextPk schemas require the caller to supply `id`; IntegerPk schemas
get an auto-assigned rowid. Output `{id, created_at}`.

## Public API

- Stored PK of the inserted row. `Integer` for auto-rowid schemas,
- INSERT + FTS reindex wrapped in one `unchecked_transaction` so a

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: chrono, crate, rusqlite, serde_json

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
