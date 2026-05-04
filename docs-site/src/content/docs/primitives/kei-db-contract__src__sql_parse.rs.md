---
title: sql_parse.rs
path: kei-db-contract/src/sql_parse.rs
dna_hash: sha256:b136b43d12daf427
language: rust
size_loc: 103
generated: by-keidocs
---

# kei-db-contract/src/sql_parse.rs

SQL parser cube: walks `migrations/*.sql`, extracts `CREATE TABLE` shapes.

## Public API

- One column extracted from a migration `CREATE TABLE` statement.
- One table extracted from a migration. Later tables with the same name override earlier ones.
- `pub fn parse_migrations_dir` — Walk `dir` recursively for `.sql` files, return parsed tables.
- `pub fn parse_sql_text` — Parse one SQL document into zero or more table definitions.

## Related

- parent: `kei-db-contract/Cargo.toml`
- imports: anyhow, serde, sqlparser, std, walkdir

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
