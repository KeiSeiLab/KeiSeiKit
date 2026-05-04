---
title: ids.rs
path: kei-cortex/src/routes/openai/ids.rs
dna_hash: sha256:c1d5e41e71d8853b
language: rust
size_loc: 53
generated: by-keidocs
---

# kei-cortex/src/routes/openai/ids.rs

ID generators for the OpenAI surface.

OpenAI uses prefixed-uuid ids for objects (`chatcmpl-...`, `resp_...`,
`run_...`). We mirror that convention so frontends that pattern-match
on the prefix continue to work.

## Public API

- `pub fn short_id` — Short hex slug for embedding in object ids — first 24 hex chars of a
- `pub fn chat_completion_id` — `chatcmpl-<24hex>` — POST /v1/chat/completions completion id.
- `pub fn tool_call_id` — `call_<24hex>` — assistant.tool_calls[*].id.
- `pub fn run_id` — `run_<24hex>` — POST /v1/runs response id.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: uuid

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
