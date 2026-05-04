---
title: cli.rs
path: kei-projects-index/src/cli.rs
dna_hash: sha256:96d07e5d16addfe8
language: rust
size_loc: 95
generated: by-keidocs
---

# kei-projects-index/src/cli.rs

CLI command handlers — keeps `main.rs` ≤ 30 LOC by holding the four
sub-command implementations as one cube.

## Public API

- `pub fn cmd_init` — `init` — open / create DB and apply schema.
- `pub fn cmd_rebuild` — `rebuild` — walk root and refresh all rows.
- `pub fn cmd_list` — `list` — print all rows as JSON.
- `pub fn cmd_get` — `get` — print one row by path.

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: kei_projects_index, rusqlite, std

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
