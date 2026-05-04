---
title: stream.rs
path: kei-llm-llamacpp/src/stream.rs
dna_hash: sha256:d81f8ce2b80ea770
language: rust
size_loc: 92
generated: by-keidocs
---

# kei-llm-llamacpp/src/stream.rs

Stream — line-buffered token streaming from `llama-cli`.

`llama-cli` emits one token per line when launched with the right
flags. We collect the lines via `Runner::run_stream` and convert each
into a `Chunk`, terminating with `done: true`.

Caller cancellation: the spec asks for "drop on caller cancel". The
Runner is owned by us; if the future returned by `stream()` is
dropped, the underlying child is dropped too — Tokio's process
handle sends SIGKILL on Drop by default.

## Public API

- One streaming token (or final marker).
- `pub fn build_stream_args` — Build argv for a streaming generate call.
- Run a streaming generate; return the full chunk vector.
- `pub fn lines_to_chunks` — Convert raw token lines to typed `Chunk`s plus a final done marker.
- Footer lines are ignored when assembling chunks.

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
