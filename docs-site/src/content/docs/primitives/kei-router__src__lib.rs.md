---
title: lib.rs
path: kei-router/src/lib.rs
dna_hash: sha256:556c4f0ef7beed4c
language: rust
size_loc: 31
generated: by-keidocs
---

# kei-router/src/lib.rs

kei-router — two routing concerns under one crate:

1. **NL query → tool-call dispatch** (LBM port; original purpose).
Public API: [`Router::new`] / [`Router::route`] / [`Router::add_dynamic`].

2. **Multi-provider LLM abstraction** (v0.40 Wave 32).
Public API: [`LlmRouter`] / [`Provider`] trait / per-provider impls.
See INTEGRATION.md for orchestrator wiring guide.

Constructor Pattern: one cube = one file. Tool router and LLM router are
independent cubes — they share crate metadata only.

## Related

- parent: `kei-router/Cargo.toml`

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
