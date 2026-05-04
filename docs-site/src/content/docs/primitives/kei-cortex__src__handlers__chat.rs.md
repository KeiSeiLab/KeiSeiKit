---
title: chat.rs
path: kei-cortex/src/handlers/chat.rs
dna_hash: sha256:2001613921ec717c
language: rust
size_loc: 119
generated: by-keidocs
---

# kei-cortex/src/handlers/chat.rs

`POST /api/v1/cortex/pet/:user_id/chat` — agentic streaming chat endpoint.

Pipeline: validate → discover CLAUDE.md/AGENTS.md → match `/skill-name` →
build system prompt → pick provider → run tool loop → translate
`LoopEvent` to SSE → wire client-disconnect to oneshot cancel.

Cost recording (Wave 40): a token-usage accumulator wraps the model
invoker; after the agentic loop emits `Done`, we spawn a blocking
task to write the row to kei-ledger via `chat_cost::record_chat_cost`.
See `chat_stream.rs` for the actual wiring; this file owns the
handler shell and request validation only.

## Public API

- JSON request body.
- Optional `?provider=<name>` selector.
- Type alias for the axum SSE response this handler returns.
- Handler entry point. Validates inputs synchronously, then returns SSE.
- Discover context + match skill + build augmented system prompt.
- Query param wins; else fallback to config default.
- Confirm the provider is registered (env had its key).
- Character ceiling for chat messages. Prevents runaway prompt injection

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, futures, kei_router, serde, std, uuid

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
