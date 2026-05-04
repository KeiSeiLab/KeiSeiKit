---
title: lib.rs
path: kei-tty/src/lib.rs
dna_hash: sha256:d720421c516f8db1
language: rust
size_loc: 18
generated: by-keidocs
---

# kei-tty/src/lib.rs

`kei-tty` — terminal UI client for the local kei-cortex daemon.

Constructor pattern:
* [`types`] — wire types (SSE `ChatEvent` enum + request body).
* [`client`] — async HTTP/SSE client (`chat_stream`).
* [`app`]    — TUI state machine (`App` + tokio::select! loop).
* [`ui`]     — ratatui frame rendering (read-only over `&App`).
* [`keys`]   — keyboard event → state-transition mapping.

Each module is independently testable. The crate has both a `lib` (for
integration tests) and a `bin` (`main.rs`) entry point.

## Related

- parent: `kei-tty/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
