---
title: search.rs
path: kei-discover/src/search.rs
dna_hash: sha256:54c85648a2b39b3c
language: rust
size_loc: 25
generated: by-keidocs
---

# kei-discover/src/search.rs

`search` — FTS5 match over `slug` + `description`.

Thin wrapper around `kei_entity_store::verbs::search` that decodes
the JSON `results` array into typed `Entry` values. An empty query
is rejected with `InvalidInput` before dispatch (the engine enforces
the same rule but we surface the typed variant eagerly).

## Related

- parent: `kei-discover/Cargo.toml`
- imports: crate, kei_entity_store, rusqlite, serde_json

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
