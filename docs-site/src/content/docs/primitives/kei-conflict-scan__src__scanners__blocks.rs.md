---
title: blocks.rs
path: kei-conflict-scan/src/scanners/blocks.rs
dna_hash: sha256:37f883bb2c694927
language: rust
size_loc: 76
generated: by-keidocs
---

# kei-conflict-scan/src/scanners/blocks.rs

Block-duplication detector (>70% text overlap).

Uses shingled-word Jaccard similarity — cheap and deterministic,
no ML / embeddings. Flags pairs above threshold.

## Related

- parent: `kei-conflict-scan/Cargo.toml`
- imports: crate, std

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
