---
title: chat_stream_ctx.rs
path: kei-cortex/src/handlers/chat_stream_ctx.rs
dna_hash: sha256:6fc3e0477673f6ab
language: rust
size_loc: 46
generated: by-keidocs
---

# kei-cortex/src/handlers/chat_stream_ctx.rs

Side-channel state captured by `chat_stream::run_loop_stream` for
the post-`Done` callbacks.

Constructor Pattern: extracted into a sibling cube so
`chat_stream.rs` can stay under the 200-LOC ceiling now that the
Hermes P2.2.b memory-nudge wiring is in place. Two structs live
here: `ChatCostCtx` (cost-recording row) and `MemoryNudgeCtx`
(memory-review trigger). Both are `Clone` so the `stream!` macro
can move them into the captured-state set without lifetime gymnastics.

## Public API

- Captures all post-loop side-channel state for cost recording.
- Conversation id passed by the client (raw, not the chat-prefixed
- Token-event tracker handle. `None` when the configured DB path
- Side-channel state for the post-`Done` memory-nudge call. Owned by

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, kei_token_tracker, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
