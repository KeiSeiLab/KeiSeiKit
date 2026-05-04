---
title: chat_events.rs
path: kei-cortex/src/handlers/chat_events.rs
dna_hash: sha256:767d58ecbb44b223
language: rust
size_loc: 40
generated: by-keidocs
---

# kei-cortex/src/handlers/chat_events.rs

Single-event SSE constructors for the chat handler.

Constructor Pattern: extracted from `chat.rs` so the parent file
stays under the 200-LOC ceiling now that Wave 40 added the cost
recording side-channel. Each function maps one shape of internal
event into the JSON payload axum's `Sse` will frame.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, serde_json

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
