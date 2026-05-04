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
