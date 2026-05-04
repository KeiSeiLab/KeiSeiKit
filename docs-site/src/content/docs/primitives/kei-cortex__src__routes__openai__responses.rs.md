---
title: responses.rs
path: kei-cortex/src/routes/openai/responses.rs
dna_hash: sha256:0897994dbae1218c
language: rust
size_loc: 212
generated: by-keidocs
---

# kei-cortex/src/routes/openai/responses.rs

POST /v1/responses, GET /v1/responses/{id}, DELETE /v1/responses/{id}.

Stateful sibling of chat-completions — `previous_response_id` chains
turns server-side. State lives in the in-memory `SessionStore`.

HERMES-MIGRATION P1.1.c: replaces `stub_agent_reply` with real loop
via `agent_runner::collect_reply` (sync) + `stream_events` +
`stream_forwarder::forward_responses` (stream). Mirrors P1.1.b.

## Public API

- Default system prompt when `instructions` is absent. Mirrors
- `POST /v1/responses`. Stream or sync depending on `req.stream`.
- `GET /v1/responses/{id}`.
- `DELETE /v1/responses/{id}`.
- `instructions` wins; blank or missing falls back to `DEFAULT_SYSTEM`.
- Build agent prompt: prior response output (chained) + current `input`.
- Sync — run loop to completion, pack into `ResponseObject` envelope.
- Stream — drive loop and forward `LoopEvent`s as SSE via
- Persist response by id so GET / continuation work.
- Single `output[]` content block in the JSON envelope.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, serde_json, tokio_util

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
