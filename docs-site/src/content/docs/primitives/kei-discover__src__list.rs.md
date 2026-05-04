---
title: list.rs
path: kei-discover/src/list.rs
dna_hash: sha256:b97e67006086a889
language: rust
size_loc: 41
generated: by-keidocs
---

# kei-discover/src/list.rs

`list_available` — return entries that have NOT been installed.

Runs a direct `SELECT` rather than `kei_entity_store::verbs::list`
because the generic verb does not filter by column; we need
`WHERE installed = 0` to hide already-installed entries. Ordering is
`id DESC` for consistency with the engine's `list` convention.

## Related

- parent: `kei-discover/Cargo.toml`
- imports: crate, rusqlite

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
