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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
