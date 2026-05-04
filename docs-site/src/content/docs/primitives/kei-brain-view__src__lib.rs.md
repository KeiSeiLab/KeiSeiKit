---
title: lib.rs
path: kei-brain-view/src/lib.rs
dna_hash: sha256:e58ca38f4e09aa33
language: rust
size_loc: 25
generated: by-keidocs
---

# kei-brain-view/src/lib.rs

kei-brain-view — read-only visualizer of the kei-ledger taxonomy graph.

Wave 14 concept: turns the SQLite `agents` table into an in-memory
`Graph` and renders it as ASCII tree, summary stats, or a DNA-centric
lineage view. NO writes to the ledger. NO new data sources.

Constructor Pattern: each sub-module owns one primitive (error, graph,
render, stats, lineage). `lib.rs` is a pure re-export surface so the
binary and integration tests share the same types.

## Related

- parent: `kei-brain-view/Cargo.toml`

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
