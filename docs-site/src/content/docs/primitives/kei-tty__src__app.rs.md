---
title: app.rs
path: kei-tty/src/app.rs
dna_hash: sha256:576d98c1fc4ec0d0
language: rust
size_loc: 157
generated: by-keidocs
---

# kei-tty/src/app.rs

TUI state machine — pure data, no I/O.

Owns the chat history, the current input buffer, scroll position, and
the "in-flight" flag (true while we are draining an SSE stream).
The actual event loop lives in [`crate::runner`] which `tokio::select!`s
over keyboard events and a channel of [`ChatEvent`]s shovelled in by
the daemon client task.

## Public API

- `pub const HISTORY_CAP` — Maximum number of message lines retained in history (older ones are
- One persisted history entry. `kind` drives the colour in [`crate::ui`].
- All UI state. Cheap to clone on a per-frame basis; cloning is avoided
- `pub fn push_line` — Append a new history line, evicting the oldest if cap exceeded.
- `pub fn apply_event` — Apply a parsed [`ChatEvent`] to the state machine.
- Move the in-progress streaming buffer into history (called on Done /

## Related

- parent: `kei-tty/Cargo.toml`
- imports: crate

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
