---
title: summary.rs
path: kei-brain-view/src/summary.rs
dna_hash: sha256:4cae613d285174e9
language: rust
size_loc: 31
generated: by-keidocs
---

# kei-brain-view/src/summary.rs

Summary rendering over kei-dna-index stats.

Constructor Pattern: one file = one responsibility (render summary).
Thin formatter — the aggregation itself lives in `kei-dna-index::stats`.

## Public API

- `pub fn render_summary` — Format the DNA-index summary block as a single text blob.

## Related

- parent: `kei-brain-view/Cargo.toml`
- imports: crate, kei_dna_index, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
