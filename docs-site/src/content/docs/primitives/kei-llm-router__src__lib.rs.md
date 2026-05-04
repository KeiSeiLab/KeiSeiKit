---
title: lib.rs
path: kei-llm-router/src/lib.rs
dna_hash: sha256:55c31b5f14d7fc2e
language: rust
size_loc: 36
generated: by-keidocs
---

# kei-llm-router/src/lib.rs

kei-llm-router — UNIVERSAL local-LLM backend selector.

Wave 60 of the local-LLM stack. Glues:
* W55 `kei-model`         — model registry (catalog + selector)
* W56 `kei-machine-probe` — host hardware/OS detection
* W57 `kei-llm-ollama`    — Ollama daemon HTTP adapter
* W58 `kei-llm-llamacpp`  — llama.cpp shell-out adapter
* W59 `kei-llm-mlx`       — Apple Silicon MLX adapter

Public surface:
* `decide(...)` — pure decision over pre-probed candidates.
* `route(...)`  — async, probes live state then decides.
* `health::check_all()` — health-probe every backend in parallel.
* `discovery::discover_models(...)` — ask each backend "have model X?"

Hard constraints (per task spec):
* NO direct subprocess spawns (delegate to W57/W58/W59).
* NO async runtime owned by this crate (the binary's `tokio::main`
runs the show; lib functions are `async fn`).
* NO new workspace deps; reuse the existing set.

Distinct from `kei-router` (the REMOTE provider router for Anthropic/
OpenAI/Kimi etc.). This crate is for LOCAL backend selection only.

## Related

- parent: `kei-llm-router/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
