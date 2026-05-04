---
title: cli.rs
path: kei-llm-router/src/cli.rs
dna_hash: sha256:35b6425ed1132b3c
language: rust
size_loc: 135
generated: by-keidocs
---

# kei-llm-router/src/cli.rs

Clap structs + subcommand handlers for the four-verb surface.

Constructor Pattern: ONE responsibility — define the user-facing CLI
and own its handlers. The `handlers` submodule keeps the actual I/O
(probe, route, list-backends, which) close to its argument structs
while leaving `main.rs` ≤30 LOC.

Subcommands (per task spec):
1. `probe`           — passthrough to `kei_machine_probe::probe`.
2. `route`           — full route decision; supports `--require-local`.
3. `list-backends`   — health-check all 3 backends, JSON table out.
4. `which --model X` — pure discovery, no health probe.

## Public API

- Run the W56 machine probe and emit JSON.
- Resolve the best local backend for a given model id.
- Emit a JSON list of `BackendHealth` entries for every backend.
- Pure discovery — which backends claim to have `<model_id>`?
- Skip the four `which` + `--version` shell-outs (CI / fast path).
- Canonical model id (e.g. `llama-3-70b-local`, `qwen3:4b`).
- Optional role tag for budget / role-default lookup.
- Optional micro-cents budget (used for registry-side filtering).
- Refuse any non-local backend (no Anthropic / OpenAI fallback).
- Model id to query — no health check, just discovery.

## Related

- parent: `kei-llm-router/Cargo.toml`
- imports: clap, crate

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
