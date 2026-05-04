---
title: main.rs
path: kei-llm-llamacpp/src/main.rs
dna_hash: sha256:03880fc80cb77806
language: rust
size_loc: 17
generated: by-keidocs
---

# kei-llm-llamacpp/src/main.rs

kei-llm-llamacpp — CLI dispatcher (thin).

Each subcommand maps to one helper in `dispatch.rs`. Errors flow
through `Error::exit_code()` so the harness sees the canonical
exit-code surface.

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: clap, kei_llm_llamacpp, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
