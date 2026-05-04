---
title: search.rs
path: kei-chat-store/src/search.rs
dna_hash: sha256:2b451189077de846
language: rust
size_loc: 38
generated: by-keidocs
---

# kei-chat-store/src/search.rs

FTS over messages.

Layer-A convergence (2026-04-23): delegates to
`kei_entity_store::verbs::search` using `MESSAGES_SCHEMA`. The engine
handles FTS5 JOIN + rank ordering; this module maps the generic
JSON result back to typed `ChatMessage` rows for legacy callers.
Per-message `cost` is persisted (engine `RealDefault` field);
`row_to_message` reads it back as f64.

## Related

- parent: `kei-chat-store/Cargo.toml`
- imports: anyhow, crate, kei_entity_store, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
