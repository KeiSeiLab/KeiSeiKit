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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
