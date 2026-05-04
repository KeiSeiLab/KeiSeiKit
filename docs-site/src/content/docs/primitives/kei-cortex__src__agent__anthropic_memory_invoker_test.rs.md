---
title: anthropic_memory_invoker_test.rs
path: kei-cortex/src/agent/anthropic_memory_invoker_test.rs
dna_hash: sha256:555900dfc722f9c5
language: rust
size_loc: 100
generated: by-keidocs
---

# kei-cortex/src/agent/anthropic_memory_invoker_test.rs

Inline unit tests for `anthropic_memory_invoker.rs`.

Constructor Pattern: extracted to a sibling so the parent stays
under the 200-LOC ceiling. Tests cover the pure helpers
(body-building, text extraction, error-reply prefix discipline);
HTTP behaviour is covered by integration tests that use wiremock.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
