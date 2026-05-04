---
title: error.rs
path: kei-cortex/src/routes/openai/error.rs
dna_hash: sha256:c63cd648edc336d3
language: rust
size_loc: 92
generated: by-keidocs
---

# kei-cortex/src/routes/openai/error.rs

OpenAI-style error envelope `{ "error": { message, type, code } }`.

Local to the `/v1/*` surface so we can match the OpenAI wire format
exactly without leaking it into the existing kei-cortex `AppError`
(which uses the `{ "error": { code, message } }` shape).

## Public API

- Local error type for `/v1/*` handlers. Each variant becomes an

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
