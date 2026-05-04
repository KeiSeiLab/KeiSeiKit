---
title: summary.rs
path: kei-brain-view/src/summary.rs
dna_hash: sha256:4cae613d285174e9
language: rust
size_loc: 31
generated: by-keidocs
---

# kei-brain-view/src/summary.rs

Summary rendering over kei-dna-index stats.

Constructor Pattern: one file = one responsibility (render summary).
Thin formatter — the aggregation itself lives in `kei-dna-index::stats`.

## Public API

- `pub fn render_summary` — Format the DNA-index summary block as a single text blob.

## Related

- parent: `kei-brain-view/Cargo.toml`
- imports: crate, kei_dna_index, rusqlite

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
