---
title: dispatch.rs
path: kei-llm-llamacpp/src/dispatch.rs
dna_hash: sha256:cd67bbf567e00c9f
language: rust
size_loc: 158
generated: by-keidocs
---

# kei-llm-llamacpp/src/dispatch.rs

Dispatch — map a parsed `Cmd` to its concrete behaviour.

Each handler is ≤30 LOC. They share a tiny set of helpers
(json print, error print) so the main.rs entry stays trivial.

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: kei_llm_llamacpp, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
