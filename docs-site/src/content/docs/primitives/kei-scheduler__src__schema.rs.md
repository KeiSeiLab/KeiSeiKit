---
title: schema.rs
path: kei-scheduler/src/schema.rs
dna_hash: sha256:9602566af1697854
language: rust
size_loc: 52
generated: by-keidocs
---

# kei-scheduler/src/schema.rs

kei-scheduler EntitySchema — declarative spec consumed by
`kei_entity_store::Store`.

Schema matches the sibling pattern (kei-task, kei-chat-store): one
primary table with standard CRUD fields plus scheduler-specific
trigger + run-tracking columns. The `name` UNIQUE constraint rides
the engine's `custom_migrations` slot because `FieldDef` doesn't
expose a UNIQUE flag — a unique index on the column provides the
same semantics.

## Public API

- Full schema list passed to the engine when opening a Store. Declared

## Related

- parent: `kei-scheduler/Cargo.toml`
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
