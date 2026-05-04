---
title: stats.rs
path: kei-discover/src/stats.rs
dna_hash: sha256:0fb1e4b71ba8b2f4
language: rust
size_loc: 31
generated: by-keidocs
---

# kei-discover/src/stats.rs

`stats` — aggregate counts (total / installed / available).

One-row SELECT with conditional SUMs. `available` equals
`total - installed`, kept as an explicit field so CLI users don't
have to subtract.

## Related

- parent: `kei-discover/Cargo.toml`
- imports: crate, rusqlite, serde

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
