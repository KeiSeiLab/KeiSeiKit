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
