---
title: server_handle_drop_kills.rs
path: kei-llm-llamacpp/tests/server_handle_drop_kills.rs
dna_hash: sha256:e83f77dbad0a8b04
language: rust
size_loc: 32
generated: by-keidocs
---

# kei-llm-llamacpp/tests/server_handle_drop_kills.rs

Spawn a mock-backed ServerHandle, drop it, assert the kill flag flips.
Mock-backed handles flip an `Arc<Mutex<bool>>` instead of sending a
signal; that lets us prove Drop fired without running a real child.

## Related

- parent: `kei-llm-llamacpp/tests`
- imports: kei_llm_llamacpp, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
