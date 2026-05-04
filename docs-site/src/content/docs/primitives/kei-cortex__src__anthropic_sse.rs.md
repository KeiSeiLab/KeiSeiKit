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
