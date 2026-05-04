---
title: sse.rs
path: kei-cortex/src/routes/openai/sse.rs
dna_hash: sha256:1ddecc091e933304
language: rust
size_loc: 161
generated: by-keidocs
---

# kei-cortex/src/routes/openai/sse.rs

SSE plumbing for /v1/chat/completions (stream=true), /v1/responses
(stream=true), and /v1/runs/{id}/events.

Implements Hermes' `kei.tool.progress` custom event (Hermes #6972)
so frontends can render tool execution UI without inferring it from
`delta.content` text. Keepalive is 30 s — short enough for nginx /
Cloudflare default 60 s read-timeouts, long enough to not spam
quiet streams.

Channel capacity is 64 — back-pressures the agent loop if the
client is slow without dropping events.

## Public API

- `pub const CHANNEL_CAPACITY` — Channel capacity for the agent → SSE writer pipe.
- `pub const KEEPALIVE_SECS` — Keepalive interval. Half a typical 60 s reverse-proxy read-timeout.
- One event the agent loop wants to push. The variant determines
- Streamed `delta.content` text — sent as the default chat-completion
- Tool-progress notification — sent as a `kei.tool.progress` event
- Final usage block — caller flushes after this and sends [DONE].
- Payload for `kei.tool.progress` events.
- "start" | "delta" | "done"
- `pub fn build_sse` — Build a paired `(sender, sse-response)` for streaming.
- `pub fn sse_from_rx` — Build an SSE response from an existing `Receiver<AgentChunk>`.
- Translate the channel of `AgentChunk` into a stream of SSE `Event`s.
- Serialise a `kei.tool.progress` event with custom event-name.
- `pub fn tool_progress` — Convenience constructor for tool-progress events. `phase` ∈
- Unix-time milliseconds without panicking. Falls back to 0 if the

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: async_stream, axum, futures, serde, std, tokio, tokio_stream

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
