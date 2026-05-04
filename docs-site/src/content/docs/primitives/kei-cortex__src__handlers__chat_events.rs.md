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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
