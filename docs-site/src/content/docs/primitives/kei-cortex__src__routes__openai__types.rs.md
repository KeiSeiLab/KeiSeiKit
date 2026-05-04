---
title: types.rs
path: kei-cortex/src/routes/openai/types.rs
dna_hash: sha256:8cc04412a0f144c0
language: rust
size_loc: 194
generated: by-keidocs
---

# kei-cortex/src/routes/openai/types.rs

OpenAI 2024-10-01 wire-format DTOs for /v1/* endpoints.

Constructor Pattern: ONE responsibility — serde shapes. No business
logic, no IO. Names match the OpenAI JSON keys verbatim so frontends
(Open WebUI, LobeChat, LibreChat, …) deserialise without translation.

## Public API

- POST /v1/chat/completions request body.
- One chat-message turn. `role` ∈ {system, user, assistant, tool}.
- OpenAI tool descriptor (function-calling schema).
- Tool-call as emitted in `assistant.tool_calls[]`.
- JSON-serialised arguments string (OpenAI quirk — not an object).
- Non-stream response body.
- Token-usage block. Names match OpenAI exactly.
- POST /v1/responses request body. Stateful via `previous_response_id`.
- Stored / returned response object.
- POST /v1/runs request — accepted (202) and processed asynchronously.
- 202 body returned from POST /v1/runs.
- GET /v1/models response item.
- OpenAI-style error envelope `{ "error": { message, type, code } }`.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: serde, serde_json

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
