---
title: stream_forwarder.rs
path: kei-cortex/src/routes/openai/stream_forwarder.rs
dna_hash: sha256:ee8834938e3a353a
language: rust
size_loc: 229
generated: by-keidocs
---

# kei-cortex/src/routes/openai/stream_forwarder.rs

Translate `tool::LoopEvent`s into SSE frames per `/v1/*` surface.

Three surfaces:
* chat-completions — `data: { delta.content }` chunks +
`kei.tool.progress` events (Hermes #6972).
* responses        — `response.output_text.delta` + `response.completed`.
* runs (P1.1.d)    — per-event `Vec<AgentChunk>` translated by
`run_agent::run_real`, then mapped to `run.message.delta` /
`run.completed` by `runs::run_event_for`.

## Public API

- `pub fn forward_chat_completions` — Forward a stream of `LoopEvent`s as OpenAI chat-completion chunks.
- Map ONE `LoopEvent` to ≥0 SSE chat-completion frames. `final_usage`
- `pub fn forward_responses` — Forward the loop stream as `/v1/responses` SSE frames.
- `pub fn forward_runs` — P1.1.d: per-event translator for `/v1/runs`. The runs surface

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, futures, serde_json, std, tokio, tokio_stream

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
