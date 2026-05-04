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
