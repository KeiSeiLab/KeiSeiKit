---
title: cluster.rs
path: kei-dna-index/src/cluster.rs
dna_hash: sha256:78efbee72361bf72
language: rust
size_loc: 50
generated: by-keidocs
---

# kei-dna-index/src/cluster.rs

Clustering over DNAs by scope / body / role+caps.

Constructor Pattern: one file = one responsibility (cluster grouping).

## Public API

- Group rows by the selected key, dropping singleton groups.

## Related

- parent: `kei-dna-index/Cargo.toml`
- imports: crate, rusqlite, serde, std

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
