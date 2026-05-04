---
title: schema.rs
path: kei-memory-postgres/src/schema.rs
dna_hash: sha256:c8dab1f2228ac63d
language: rust
size_loc: 58
generated: by-keidocs
---

# kei-memory-postgres/src/schema.rs

Schema bootstrap. One idempotent `CREATE TABLE IF NOT EXISTS` plus
two indexes. Anything richer (partitioning, GIN on tags, FTS) is a
caller's choice — keep this primitive minimal.

## Public API

- `pub const SCHEMA_SQL` — SSoT DDL applied by [`apply_schema`]. Public so external callers
- Run [`SCHEMA_SQL`] against the supplied client. Idempotent — safe to

## Related

- parent: `kei-memory-postgres/Cargo.toml`
- imports: crate, tokio_postgres

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
