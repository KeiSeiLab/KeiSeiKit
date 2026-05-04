---
title: store.rs
path: kei-task/src/store.rs
dna_hash: sha256:6b7bd53b11a06395
language: rust
size_loc: 101
generated: by-keidocs
---

# kei-task/src/store.rs

Task store — thin shim over `kei_entity_store::Store`.

Layer-A convergence pilot (2026-04-23): generic CRUD (create / get /
update) now runs through `kei_entity_store::verbs::*` using the
declarative `TASK_SCHEMA`. Public surface is preserved byte-for-byte
so existing integration tests and callers (`atoms::create`,
`milestones`, `deps`, `search`) compile unchanged.

## Related

- parent: `kei-task/Cargo.toml`
- imports: anyhow, crate, kei_entity_store, rusqlite, serde_json, std

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
