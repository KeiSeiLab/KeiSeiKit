---
title: chat_completions.rs
path: kei-cortex/src/routes/openai/chat_completions.rs
dna_hash: sha256:505928664af9e8ea
language: rust
size_loc: 222
generated: by-keidocs
---

# kei-cortex/src/routes/openai/chat_completions.rs

POST /v1/chat/completions handler. Stateless by default; opt-in
continuity via `X-Kei-Session-Id`. Streaming delegates per-event
SSE translation to `stream_forwarder::forward_chat_completions`.

## Public API

- Handler entry point — dispatches to `handle_stream` or `handle_sync`.
- Streaming path — forward each `LoopEvent` as its own SSE frame
- Tee `upstream` into a fresh receiver: forward every event and

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, tokio, tokio_util

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
