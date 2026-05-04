---
title: lib.rs
path: kei-llm-mlx/src/lib.rs
dna_hash: sha256:5a25c338c9952c5a
language: rust
size_loc: 40
generated: by-keidocs
---

# kei-llm-mlx/src/lib.rs

kei-llm-mlx — public library surface (Wave 59).

Adapter to Apple MLX inference framework for native Apple Silicon
local-LLM hosting. Wraps the canonical `mlx_lm.generate` and
`mlx_lm.server` Python CLIs (installed via `pip install mlx-lm`).

Position: parallel sibling of `kei-llm-ollama` (W57) and
`kei-llm-llamacpp` (W58); glued together by `kei-llm-router` (W60).

Design decisions:
- Shell-out, NOT PyO3 — keeps the crate Apple-Silicon-only by gate,
not by conditional compilation, and avoids dragging Python build
deps into Rust.
- Constructor Pattern — every responsibility (platform gate, binary
discovery, model list, generate, stream parse, server spawn,
error) lives in its own cube ≤200 LOC.
- Runner trait — every subprocess goes through `runner::Runner` so
tests substitute `MockRunner` and never invoke real `mlx_lm`.

## Related

- parent: `kei-llm-mlx/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
