---
title: fallback.rs
path: kei-model/src/fallback.rs
dna_hash: sha256:e3934c19495b0f78
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-model/src/fallback.rs

`chain` — walk `fallback` field until None or cycle.

Detects cycles via a visited-set. Unknown ids halt the walk before adding
the unknown id to the chain. Returns a `Vec<Model>` in walk order with the
primary at index 0.

## Public API

- `pub fn chain` — Walk the fallback chain starting at `primary`.

## Related

- parent: `kei-model/Cargo.toml`
- imports: anyhow, crate, std

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
