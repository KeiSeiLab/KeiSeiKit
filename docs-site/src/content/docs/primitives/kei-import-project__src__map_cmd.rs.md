---
title: map_cmd.rs
path: kei-import-project/src/map_cmd.rs
dna_hash: sha256:ac6de2b468139340
language: rust
size_loc: 100
generated: by-keidocs
---

# kei-import-project/src/map_cmd.rs

map_cmd — build an architecture map of a repo by running the matcher per module.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- One row in the architecture map.
- `pub fn render_markdown` — Render entries as a markdown table.
- `pub fn render_json` — Render entries as a JSON array.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: anyhow, crate, serde, std

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
