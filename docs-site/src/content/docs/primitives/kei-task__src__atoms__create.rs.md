---
title: create.rs
path: kei-task/src/atoms/create.rs
dna_hash: sha256:84319ae93d518fc6
language: rust
size_loc: 82
generated: by-keidocs
---

# kei-task/src/atoms/create.rs

kei-task::create atom — contract in atoms/create.md.

Layer-A pilot: validates task-specific input (title non-empty,
priority enum) then delegates the INSERT + FTS reindex to
`kei_entity_store::verbs::create` through the crate-level
`TASK_SCHEMA`.

## Related

- parent: `kei-task/Cargo.toml`
- imports: crate, kei_entity_store, serde, serde_json, std

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
