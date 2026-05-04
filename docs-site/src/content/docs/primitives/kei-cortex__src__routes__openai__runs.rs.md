---
title: runs.rs
path: kei-cortex/src/routes/openai/runs.rs
dna_hash: sha256:6b475b4ef80cc2fd
language: rust
size_loc: 190
generated: by-keidocs
---

# kei-cortex/src/routes/openai/runs.rs

POST /v1/runs, GET /v1/runs/{id}/events, POST /v1/runs/{id}/stop.

`runs` is the asynchronous variant of chat-completions: POST returns
202 + run id immediately, GET subscribes to the SSE event stream,
POST /stop fires the run's `CancellationToken` so the agent loop
exits gracefully at its next checkpoint.

Constructor Pattern: state lives in `run_registry::RunRegistry`,
the real agent in `run_agent::run_real`. This file owns ONLY the
three HTTP handlers + validation + the per-chunk SSE mapper.

P1.1.d (2026-04-28): wired the spawn path to the real agent loop
via `run_agent::run_real`. The previous `run_stub` is removed.

## Public API

- `POST /v1/runs` — accept the request, allocate a run id, spawn the
- Spawn the real agent loop in the background. Mirrors P1.1.b's
- `GET /v1/runs/{id}/events` — SSE attached to the run's chunk channel.
- `POST /v1/runs/{id}/stop` — fire the run's `CancellationToken` so

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, serde_json, std, tokio, tokio_util

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
