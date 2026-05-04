---
title: telegram_smoke.rs
path: kei-notify-telegram/tests/telegram_smoke.rs
dna_hash: sha256:0b905266c3683866
language: rust
size_loc: 201
generated: by-keidocs
---

# kei-notify-telegram/tests/telegram_smoke.rs

Wiremock-only integration tests. No live HTTP, no Telegram API
calls. Covers the Bot API `sendMessage` happy path, the
`{"ok":false}` rejection path, the 5xx transport error, the
`parse_mode=HTML` body invariant, the HTML-escape invariant, and
the DNA cap surface.

## Related

- parent: `kei-notify-telegram/tests`
- imports: kei_notify_telegram, kei_runtime_core, serde_json, wiremock

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
