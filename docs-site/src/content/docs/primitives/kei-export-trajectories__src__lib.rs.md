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
