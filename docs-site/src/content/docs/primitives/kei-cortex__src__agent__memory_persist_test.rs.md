---
title: memory_persist_test.rs
path: kei-cortex/src/agent/memory_persist_test.rs
dna_hash: sha256:1f8cb8c7436e0217
language: rust
size_loc: 97
generated: by-keidocs
---

# kei-cortex/src/agent/memory_persist_test.rs

Inline unit tests for `memory_persist.rs`.

Constructor Pattern: extracted to a sibling so the parent stays
≤200 LOC. Tests cover the pure classifier and the on-disk write
path against a fresh tempfile sqlite.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: kei_pet, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
