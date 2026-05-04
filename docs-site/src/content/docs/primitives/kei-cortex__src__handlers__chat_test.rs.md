---
title: chat_test.rs
path: kei-cortex/src/handlers/chat_test.rs
dna_hash: sha256:9282901790826eed
language: rust
size_loc: 156
generated: by-keidocs
---

# kei-cortex/src/handlers/chat_test.rs

Inline unit tests for `chat.rs`.

Loop-termination paths are covered exhaustively in
`tool/tests/loop_terminates_on_max_turns.rs`. These tests focus on the
per-event SSE translation, body validation, and provider validation —
the surfaces unique to the handler layer.

After Wave 40 split, the SSE event constructors and `loop_event_to_sse`
moved to sibling cubes (`chat_events.rs`, `chat_stream.rs`); the tests
reach them via the parent `super::chat_*::` paths.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
