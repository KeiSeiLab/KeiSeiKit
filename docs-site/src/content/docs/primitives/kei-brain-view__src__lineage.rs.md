---
title: lineage.rs
path: kei-brain-view/src/lineage.rs
dna_hash: sha256:39d8d43c21861865
language: rust
size_loc: 78
generated: by-keidocs
---

# kei-brain-view/src/lineage.rs

Ancestor + descendant walk for a single DNA.

Constructor Pattern: one cube = `Lineage` struct + BFS descent + parent
chain walk. Both walks cycle-safe via `visited` set + `MAX_TREE_DEPTH`.

## Public API

- `pub fn lineage` — Resolve a DNA prefix and return its ancestors + descendants.

## Related

- parent: `kei-brain-view/Cargo.toml`
- imports: crate, serde, std

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
