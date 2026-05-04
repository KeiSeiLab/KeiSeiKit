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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
