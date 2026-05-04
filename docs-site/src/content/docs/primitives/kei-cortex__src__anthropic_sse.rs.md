---
title: anthropic_sse.rs
path: kei-cortex/src/anthropic_sse.rs
dna_hash: sha256:6cb6ecd6d94b734d
language: rust
size_loc: 119
generated: by-keidocs
---

# kei-cortex/src/anthropic_sse.rs

Incremental SSE parser for Anthropic Messages streaming responses.

SSE frames are separated by `\n\n`; a single chunk may contain a partial
frame, so `SseParser` buffers across `push` calls and emits every text
delta as it completes. Non-text events (`message_start`, `ping`, etc.)
are skipped; the parser intentionally only surfaces `text_delta` payloads.

## Public API

- Anthropic content-block-delta payload (only the subfield we care about).
- Maximum buffer size per SSE frame — guards against a runaway upstream
- Error returned by `SseParser::push` when the buffer cap is exceeded.
- Incremental SSE parser — SSE frames are separated by `\n\n`.
- `pub fn push` — Consume a byte chunk, return every text delta completed in this push.
- Parse a single SSE frame and return the text delta if present.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: bytes, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
