---
title: runner.rs
path: kei-tty/src/runner.rs
dna_hash: sha256:bccfab45af83be35
language: rust
size_loc: 93
generated: by-keidocs
---

# kei-tty/src/runner.rs

Async event loop — couples the [`App`] state machine to crossterm key
events and the daemon SSE stream over a `tokio::mpsc` channel.

## Public API

- Run the TUI event loop until the user presses Ctrl+D / Ctrl+C twice.
- Hand a [`KeyEvent`](crossterm::event::KeyEvent) to [`handle_key`] and
- Spawn the background daemon-client task for a single send.

## Related

- parent: `kei-tty/Cargo.toml`
- imports: anyhow, crate, crossterm, futures, ratatui, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
