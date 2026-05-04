---
title: chat_memory_nudge_test.rs
path: kei-cortex/src/handlers/chat_memory_nudge_test.rs
dna_hash: sha256:8b6176649e05a9e5
language: rust
size_loc: 112
generated: by-keidocs
---

# kei-cortex/src/handlers/chat_memory_nudge_test.rs

Inline unit tests for `chat_memory_nudge.rs`.

Constructor Pattern: extracted to a sibling so the parent stays
≤200 LOC. Tests cover the context-builder and verify the wiring
ends up with both `invoker` and `persist` populated (regression
against the prior dead-code state).

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
