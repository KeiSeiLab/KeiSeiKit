---
title: dna_class.rs
path: kei-model-router/src/dna_class.rs
dna_hash: sha256:64d7e69d98ece8ea
language: rust
size_loc: 136
generated: by-keidocs
---

# kei-model-router/src/dna_class.rs

Task-class DNA extraction.

Full DNA format: `<role>::<caps>::<sha8-scope>::<sha8-body>-<nonce8>`.
Examples:
code-implementer-rust::?::e3929e37::041b7526-674c5cf3
edit-local::NG-FW-FD-CP-CG-TG-ND-RF::5435F821::AC73A6A3-b3d36aa6

Three abstraction levels for posterior aggregation:

1. `full_dna` — every spawn unique (random nonce). One observation per row.
2. `task_class_dna` — strip `-<nonce8>`. Same prompt re-runs cluster.
3. `agent_class_dna` — strip `::<body8>-<nonce8>`. Same agent at same scope,
different prompts cluster. Highest-level routable identity.

Constructor Pattern: this cube is purely lexical. No I/O, no SQL.

## Public API

- `pub fn task_class_dna` — Strip trailing `-<nonce8>` from full DNA. Mirrors the SQL VIRTUAL column
- `pub fn agent_class_dna` — Strip `::<body8>-<nonce8>` from full DNA. Yields role+caps+scope identity.
- `pub fn role` — First `::` separated component — the substrate role slug.
- `pub fn caps` — Second `::` separated component — capability bundle codes.
- `pub fn scope_sha` — Third `::` separated component — scope sha-8.
- `pub fn body_sha` — Body sha-8 from a task-class DNA (after agent_class).

## Related

- parent: `kei-model-router/Cargo.toml`

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
