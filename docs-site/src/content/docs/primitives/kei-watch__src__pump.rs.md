---
title: pump.rs
path: kei-watch/src/pump.rs
dna_hash: sha256:c2f282d15e94f470
language: rust
size_loc: 72
generated: by-keidocs
---

# kei-watch/src/pump.rs

Single-threaded pump: reads `Result<notify::Event>` from notify's
channel, maps + debounces, pushes canonical [`Event`] to the output
channel consumed by `next_event` / `drain`.

Exactly one thread is spawned per [`crate::Watcher`] instance. The
thread exits cleanly when notify's sender is dropped (closing the
input channel), which happens when the `notify::RecommendedWatcher`
is dropped inside [`crate::Watcher::drop`].

## Public API

- `pub fn spawn` — Spawn the pump thread.

## Related

- parent: `kei-watch/Cargo.toml`
- imports: crate, notify, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
