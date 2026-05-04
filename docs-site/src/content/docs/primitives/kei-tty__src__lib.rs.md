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
