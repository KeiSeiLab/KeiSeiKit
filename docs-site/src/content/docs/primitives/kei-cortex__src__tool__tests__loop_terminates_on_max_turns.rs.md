---
title: loop_terminates_on_max_turns.rs
path: kei-cortex/src/tool/tests/loop_terminates_on_max_turns.rs
dna_hash: sha256:a0371b8ee9d4e702
language: rust
size_loc: 136
generated: by-keidocs
---

# kei-cortex/src/tool/tests/loop_terminates_on_max_turns.rs

Validates the loop's hard termination guarantees:
- infinite tool-use stream stops at MAX_TURNS with an Error event
- cancellation via CancellationToken causes early Done
- clean end_turn stop_reason terminates immediately

Wave 44c (F-HIGH-5): cancel migrated from `oneshot::Receiver<()>`
to `tokio_util::sync::CancellationToken`.

## Public API

- Build an invoker that ALWAYS asks for one tool_use, simulating a
- A registry where `noop` always succeeds. Keeps the loop fed.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, futures, std, tokio_util

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
