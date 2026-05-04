---
title: budget.rs
path: kei-search-core/src/budget.rs
dna_hash: sha256:5a39bfdaee269910
language: rust
size_loc: 34
generated: by-keidocs
---

# kei-search-core/src/budget.rs

Budget tracker — all costs in microcents (1 USD = 1_000_000 mc).

## Public API

- `pub fn charge` — Record a cost; returns error if this push would exceed the cap.

## Related

- parent: `kei-search-core/Cargo.toml`
- imports: anyhow

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
