---
title: main.rs
path: kei-diff/src/main.rs
dna_hash: sha256:dc4acd40f370d196
language: rust
size_loc: 80
generated: by-keidocs
---

# kei-diff/src/main.rs

kei-diff CLI.

Usage:
kei-diff diff  --old <path> --new <path>       # prints RFC 6902 patch
kei-diff apply --base <path> --patch <path>    # prints result document

No external arg-parser dep — this is a two-verb tool with fixed flag sets,
hand-rolling keeps the crate zero-dep beyond serde/serde_json.

## Related

- parent: `kei-diff/Cargo.toml`
- imports: std

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
