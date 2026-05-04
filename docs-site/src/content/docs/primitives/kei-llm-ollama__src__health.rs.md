---
title: health.rs
path: kei-llm-ollama/src/health.rs
dna_hash: sha256:e3a046919cf04353
language: rust
size_loc: 47
generated: by-keidocs
---

# kei-llm-ollama/src/health.rs

Liveness probe for the Ollama daemon.

Used by the W60 router to decide whether to route to the Ollama backend
or fall back to llama.cpp / mlx. Short timeout (1s default) — never blocks
the parent for long.

## Public API

- Quick `is the daemon up?` probe. Returns `true` on 2xx /api/tags within timeout.
- Full health snapshot — version + model count.
- Fetch a complete health snapshot. `running=false` if either probe fails.

## Related

- parent: `kei-llm-ollama/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
