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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
