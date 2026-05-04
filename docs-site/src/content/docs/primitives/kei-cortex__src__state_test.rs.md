---
title: state_test.rs
path: kei-cortex/src/state_test.rs
dna_hash: sha256:29c0ff03ae1902ce
language: rust
size_loc: 122
generated: by-keidocs
---

# kei-cortex/src/state_test.rs

Inline unit tests for `state.rs`.

Constructor Pattern: extracted to a sibling so the parent stays
≤200 LOC after the Hermes P2.2.b additions (scheduler + invoker
factory plumbing) and Wave 44d resource-cap hardening (per-user
lock LRU eviction).

## Public API

- Resource-exhaustion guard: the registry MUST cap at
- LRU recency: the most recent inserts MUST survive eviction. After

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std

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
