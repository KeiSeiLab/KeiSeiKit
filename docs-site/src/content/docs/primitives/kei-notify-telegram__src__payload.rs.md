---
title: payload.rs
path: kei-notify-telegram/src/payload.rs
dna_hash: sha256:3ca39605ff9a9e66
language: rust
size_loc: 123
generated: by-keidocs
---

# kei-notify-telegram/src/payload.rs

HTML body composition for Telegram Bot API `sendMessage`.

Format: `<b>{subject}</b>\n\n{severity_emoji} {body_text}`.
The subject is wrapped in `<b>...</b>` so Telegram's HTML
parse_mode renders it bold; the body is severity-prefixed so
readers see the level at a glance.

HTML escaping: Telegram's HTML parse_mode requires `<`, `>`, `&`
to be escaped in plain content (the only tags it understands are
`b/i/u/s/code/pre/a` etc.) Without escaping, a stray `<` in either
field can either error 400 or, worse, render as a literal tag.

## Public API

- `pub fn severity_emoji` — Map a severity to its display emoji. Pure mapping, no allocation.
- `pub fn build_text` — Compose the HTML-formatted message body.
- Minimal HTML escape for Telegram parse_mode=HTML.

## Related

- parent: `kei-notify-telegram/Cargo.toml`
- imports: kei_runtime_core

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
