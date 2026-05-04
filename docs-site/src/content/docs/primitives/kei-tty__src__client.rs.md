---
title: client.rs
path: kei-tty/src/client.rs
dna_hash: sha256:9a14fd4446dd4de7
language: rust
size_loc: 134
generated: by-keidocs
---

# kei-tty/src/client.rs

Async HTTP/SSE client for the cortex daemon.

`chat_stream` opens `POST /api/v1/cortex/pet/:user_id/chat`, drains the
SSE response, and invokes a callback for every parsed [`ChatEvent`].

The SSE parser is intentionally minimal — frames are split on `\n\n`
(event terminator) and each frame has its `data:` lines concatenated.
Comment lines (starting with `:`) and `event:` / `id:` / `retry:` lines
are ignored, matching the W3C EventSource specification subset that
axum's `Sse` writer emits.

## Public API

- Dispatch a chat request and stream events to `on_event` as they arrive.
- Drain a `reqwest::Response` body as SSE frames.
- `pub fn flush_complete_frames` — Pull every complete `\n\n`-terminated frame out of `buf`, parse it, and
- Extract the `ChatEvent` carried by a single SSE frame.

## Related

- parent: `kei-tty/Cargo.toml`
- imports: anyhow, crate, futures

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
