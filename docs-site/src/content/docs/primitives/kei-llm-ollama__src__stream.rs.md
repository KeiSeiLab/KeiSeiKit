---
title: stream.rs
path: kei-llm-ollama/src/stream.rs
dna_hash: sha256:fa8a964a11846917
language: rust
size_loc: 107
generated: by-keidocs
---

# kei-llm-ollama/src/stream.rs

NDJSON stream consumer for `/api/generate` and `/api/chat` (`stream: true`).

Ollama emits one JSON object per line, terminated by an object with `done: true`.
Schema source: <https://github.com/ollama/ollama/blob/main/docs/api.md>

## Public API

- One streamed chunk from `/api/generate` or `/api/chat`.
- `pub struct NdjsonBuffer` — Buffer that splits a byte stream into newline-delimited JSON payloads.
- `pub fn push` — Push bytes; return any complete JSON lines (one per finished line).
- `pub fn decode_line` — Decode one NDJSON line into either a generate-style or chat-style chunk.
- `pub fn chunk_stream` — Convert a raw bytes-stream (from reqwest) into a stream of [`Chunk`].

## Related

- parent: `kei-llm-ollama/Cargo.toml`
- imports: bytes, crate, futures, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
