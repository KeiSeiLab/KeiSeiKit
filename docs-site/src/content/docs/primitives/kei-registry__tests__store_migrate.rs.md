---
title: store_migrate.rs
path: kei-registry/tests/store_migrate.rs
dna_hash: sha256:35c8aa43d8ee547e
language: rust
size_loc: 161
generated: by-keidocs
---

# kei-registry/tests/store_migrate.rs

Schema migration tests (v1 → v5).

Three scenarios:
1. Simulate an "old" v1 database on disk, then re-open via `open_db` and
confirm it auto-migrates to current SCHEMA_VERSION: new columns on
`blocks`, new tables `block_predicates` + `block_deps` exist.
2. Re-opening an already-current database is a no-op (idempotent).
3. Migrating from v4 to v5 adds `cleanup_findings` table + 2 indices.

## Related

- parent: `kei-registry/tests`
- imports: kei_registry, rusqlite, tempfile

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
