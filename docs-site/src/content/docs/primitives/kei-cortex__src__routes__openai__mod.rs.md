---
title: mod.rs
path: kei-cortex/src/routes/openai/mod.rs
dna_hash: sha256:bff43e2706d09d95
language: rust
size_loc: 98
generated: by-keidocs
---

# kei-cortex/src/routes/openai/mod.rs

OpenAI-compatible HTTP surface — `/v1/*` routes.

Constructor Pattern: this `mod.rs` ONLY assembles the router.
Each cube under it owns one responsibility:

* `types.rs`              wire-format DTOs
* `error.rs`              OpenAI-shaped error envelope
* `auth.rs`               Bearer-token / loopback middleware
* `translation.rs`        OpenAI ⇄ kei-cortex tool-name mapping
* `sse.rs`                SSE primitives + `kei.tool.progress`
* `stream_chunks.rs`      chat-completion stream frame builders
* `session.rs`            in-memory session continuity store
* `run_registry.rs`       in-memory `/v1/runs` slot store
* `run_agent.rs`          real agent loop (P1.1.d) — drains
`tool::LoopEvent` into `AgentChunk`s
for the SSE handler in `runs.rs`
* `ids.rs`                prefixed-uuid id generators
* `models.rs`             GET /v1/models
* `chat_completions.rs`   POST /v1/chat/completions
* `chat_helpers.rs`       chat-completions validation helpers
* `responses.rs`          POST /v1/responses + GET/DELETE
* `runs.rs`               POST /v1/runs + events + stop

State (`SessionStore`, `RunRegistry`) is held in process-global
`once_cell::Lazy` singletons (`session::global()`,
`run_registry::global()`) so the returned router is `Router<S>` for
any `S` and merges cleanly into the existing kei-cortex router
whose state is `AppState`.

## Public API

- `pub fn openai_router` — Build the `/v1/*` sub-router. Auth middleware reads `KEI_API_KEY`

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate

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
