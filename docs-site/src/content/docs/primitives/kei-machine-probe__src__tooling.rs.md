---
title: tooling.rs
path: kei-machine-probe/src/tooling.rs
dna_hash: sha256:7d5fcf71c3db4bc0
language: rust
size_loc: 50
generated: by-keidocs
---

# kei-machine-probe/src/tooling.rs

Local-LLM tooling detection.

For each binary of interest:
1. `which <bin>` — present? (non-zero exit ⇒ absent)
2. `<bin> --version` — extract version string (best-effort).

Bins probed: `ollama`, `brew`, `llama-server` (llama.cpp's HTTP
daemon — the binary kei-llm-llamacpp will spawn). All optional —
every detector returns `None` on failure rather than erroring.

## Public API

- Pull a version token from `<bin> --version` output. Tries, in order:

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: crate, regex

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
