---
title: chat_helpers.rs
path: kei-cortex/src/routes/openai/chat_helpers.rs
dna_hash: sha256:456b692fae2cd8ee
language: rust
size_loc: 143
generated: by-keidocs
---

# kei-cortex/src/routes/openai/chat_helpers.rs

Validation + session-id parsing + response-building helpers for
`chat_completions.rs`. Split out so that handler file stays under
the 200-LOC Constructor-Pattern ceiling.

## Public API

- `pub fn validate` — Reject empty model / empty messages with a 400.
- `pub fn session_id_from_headers` — Pull the optional `X-Kei-Session-Id` header. Empty / whitespace
- `pub fn collect_session_messages` — Concatenate prior session messages + the request's messages.
- `pub fn persist_turn` — Persist {request_messages, assistant_reply} into the session if a
- `pub fn build_completion_response` — Build the non-stream `ChatCompletionResponse` envelope. `usage`
- Phase-1.1 placeholder agent reply. Phase 1.1.b/c/d replaced this with

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
