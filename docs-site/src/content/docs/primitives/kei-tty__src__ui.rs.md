---
title: ui.rs
path: kei-tty/src/ui.rs
dna_hash: sha256:80bc3eb0444f9661
language: rust
size_loc: 160
generated: by-keidocs
---

# kei-tty/src/ui.rs

Ratatui frame rendering. Pure read-only function over `&App`.

Layout (vertical splits):
```text
┌─────────────────────────────────────────────┐
│ chat history (Paragraph, scrollable)        │  ~70%
├─────────────────────────────────────────────┤
│ input bar (multi-line)                      │  ~25%
├─────────────────────────────────────────────┤
│ status line                                 │  fixed 1 row
└─────────────────────────────────────────────┘
```
Tool-call boxes are rendered inline inside the chat history (yellow for
`tool_use`, green for `tool_result`).

## Public API

- `pub fn draw` — Top-level entry — draws the whole UI for one frame.
- Render the rolling chat history including the in-progress streaming
- Render the user's input bar.
- Render the status line (bottom one row).
- Map a history [`AppLine`] to a styled ratatui [`Line`].
- Per-`LineKind` (prefix, foreground colour). Hard-coded palette — no theme
- Clamp `requested` so the renderer never tries to scroll past the bottom.

## Related

- parent: `kei-tty/Cargo.toml`
- imports: crate, ratatui

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
