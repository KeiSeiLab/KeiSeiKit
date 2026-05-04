---
title: search.rs
path: kei-task/src/search.rs
dna_hash: sha256:657b770d4affef77
language: rust
size_loc: 37
generated: by-keidocs
---

# kei-task/src/search.rs

FTS5 search over tasks (title + description).

Thin shim over `kei_entity_store::verbs::search` preserved for
callers (integration tests, CLI `cmd_search`) that still want the
strongly-typed `Vec<Task>` surface.

## Related

- parent: `kei-task/Cargo.toml`
- imports: anyhow, crate, kei_entity_store, serde_json

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
