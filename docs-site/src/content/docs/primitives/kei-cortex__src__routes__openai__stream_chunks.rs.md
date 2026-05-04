---
title: stream_chunks.rs
path: kei-cortex/src/routes/openai/stream_chunks.rs
dna_hash: sha256:6e49980a0195c2f7
language: rust
size_loc: 72
generated: by-keidocs
---

# kei-cortex/src/routes/openai/stream_chunks.rs

Helpers that serialise an `AgentChunk` into a chat-completion
`data: ...` SSE frame matching the OpenAI streaming spec.

Kept separate from `sse.rs` so the SSE primitives stay generic
across chat-completions / responses / runs surfaces.

## Public API

- `pub fn content_chunk` — `data: { delta: { content } }` chunk shape used while streaming.
- `pub fn finish_chunk` — Final chunk: empty delta + `finish_reason: stop` + usage block.
- `pub fn done_sentinel` — `data: [DONE]` sentinel — emitted after the finish chunk so OpenAI

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
