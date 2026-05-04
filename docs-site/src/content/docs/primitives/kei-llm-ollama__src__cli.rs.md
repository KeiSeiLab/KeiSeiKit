---
title: cli.rs
path: kei-llm-ollama/src/cli.rs
dna_hash: sha256:0e17462c6a17a4ee
language: rust
size_loc: 107
generated: by-keidocs
---

# kei-llm-ollama/src/cli.rs

Clap definitions for the 5 subcommands.

## Public API

- List installed models (GET /api/tags).
- Single-prompt completion (POST /api/generate).
- Multi-turn chat (POST /api/chat).
- Download or update a model (POST /api/pull).
- Health probe — returns {running, version, models_count}.
- Ollama daemon URL. Local-only by default for security.
- Per-call timeout in milliseconds. Ignored for streaming flows.
- Model name (e.g. `qwen3:4b`).
- User prompt.
- Stream NDJSON chunks one per line (instead of full JSON).
- Cap response tokens (`options.num_predict`).
- Sampling temperature (`options.temperature`).
- Model name.
- Inline JSON array of `{role, content}` OR `@path/to/file.json`.
- Stream NDJSON chunks one per line.
- Cap response tokens (`options.num_predict`).
- Sampling temperature.
- Model to pull (e.g. `qwen3:4b`).

## Related

- parent: `kei-llm-ollama/Cargo.toml`
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
