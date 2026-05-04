---
title: keys.rs
path: kei-tty/src/keys.rs
dna_hash: sha256:6281640507c6d422
language: rust
size_loc: 159
generated: by-keidocs
---

# kei-tty/src/keys.rs

Keyboard handler — translates [`KeyEvent`] into [`KeyOutcome`].

Bindings:
* `Enter`                 — send the input buffer (returns [`KeyOutcome::Send`])
* `Shift+Enter`           — insert newline into the input buffer
* `Ctrl+C`                — cancel the in-flight request
* `Ctrl+D`                — exit the program
* `Ctrl+L`                — clear the visible chat (history retained)
* `PageUp` / `PageDown`   — scroll history by one page
* `Backspace`             — delete one character from input
* any printable character — append to input buffer

## Public API

- `pub enum KeyOutcome` — Result of mapping one keypress.
- The user pressed Enter and the buffer was non-empty; payload is the
- User asked to exit (Ctrl+D).
- User asked to cancel the in-flight request (Ctrl+C while streaming).
- Pure view-state edit (input buffer, scroll, clear); nothing else.
- Page step for PageUp/PageDown. Hard-coded — the renderer does not have
- `pub fn handle_key` — Top-level dispatcher.

## Related

- parent: `kei-tty/Cargo.toml`
- imports: crate, crossterm

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
