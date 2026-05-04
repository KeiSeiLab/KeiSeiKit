---
title: lib.rs
path: kei-export-trajectories/src/lib.rs
dna_hash: sha256:5a47381fef6799d3
language: rust
size_loc: 29
generated: by-keidocs
---

# kei-export-trajectories/src/lib.rs

kei-export-trajectories — public library surface.

Constructor Pattern: the binary (`main.rs`) and tests share the same
module tree by depending on this lib. External callers (e.g. a future
`kei-cortex` integration that exports on demand) get a stable Rust API
without re-implementing CLI parsing.

HERMES-MIGRATION-PLAN P0.2: emits ShareGPT-compatible JSONL so any
Hermes-aware trainer / dataset loader / HuggingFace pipeline can ingest
KeiSei agent trajectories with zero conversion work.

## Related

- parent: `kei-export-trajectories/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
