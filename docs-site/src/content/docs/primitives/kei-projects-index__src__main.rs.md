---
title: main.rs
path: kei-projects-index/src/main.rs
dna_hash: sha256:81926c534f402c1e
language: rust
size_loc: 56
generated: by-keidocs
---

# kei-projects-index/src/main.rs

kei-projects-index — CLI dispatcher.

Constructor Pattern: this binary holds clap shapes only. Every command
forwards to a function in the sibling `cli` / `query` cubes or to
`kei_projects_index::*` library APIs.

## Public API

- Create the DB file and apply the schema.
- Walk projects_root and refresh every row.
- Print all rows as a JSON array to stdout.
- Print one project row as JSON.

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: clap, std

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
