---
title: schema.rs
path: kei-task/src/schema.rs
dna_hash: sha256:59a5076c85d8d8b1
language: rust
size_loc: 69
generated: by-keidocs
---

# kei-task/src/schema.rs

kei-task EntitySchema — declarative spec consumed by
`kei_entity_store::Store` and its verb templates.

Columns match the legacy `CREATE TABLE tasks` DDL byte-for-byte so
on-disk databases created before the convergence layer continue to
work.

Task-specific secondary tables (`milestones`, `task_deps`,
`task_milestones`) ride the engine's `custom_migrations` slot — they
are not generic CRUD and keep their existing column names so
`deps.rs` / `milestones.rs` / `graph.rs` don't need to change.

## Related

- parent: `kei-task/Cargo.toml`
- imports: kei_entity_store

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
