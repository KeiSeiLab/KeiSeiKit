---
title: dispatch.rs
path: kei-cortex/src/tool/dispatch.rs
dna_hash: sha256:45e759a9a44a8a1c
language: rust
size_loc: 119
generated: by-keidocs
---

# kei-cortex/src/tool/dispatch.rs

Per-turn outcome dispatcher.

`dispatch_outcome` translates a `TurnOutcome` (model said: text-only,
tool-use, or invoker-error) into a flat `Vec<LoopEvent>` the outer
`inner_loop` can yield in order. This file is split out from
`loop_driver.rs` so each cube stays inside the Constructor Pattern
200-LOC file ceiling.

## Public API

- Translate one `TurnOutcome` into a flat sequence of events. When the
- Build the event list for a `Final` outcome.
- Build the event list for a `Continue` outcome and stash tool results

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
