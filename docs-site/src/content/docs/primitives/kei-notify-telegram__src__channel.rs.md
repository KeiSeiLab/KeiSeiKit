---
title: channel.rs
path: kei-notify-telegram/src/channel.rs
dna_hash: sha256:75b2d658f435178f
language: rust
size_loc: 134
generated: by-keidocs
---

# kei-notify-telegram/src/channel.rs

`TelegramChannel` — `NotifyChannel` impl that POSTs to the
Telegram Bot API `sendMessage` endpoint.

Constructor surface:
* [`TelegramChannel::from_env`]    — reads `TELEGRAM_BOT_TOKEN`
+ `TELEGRAM_CHAT_ID`, defaults base URL to
`https://api.telegram.org`.
* [`TelegramChannel::with_config`] — explicit base URL, token,
chat_id (used in wiremock tests).

Wire format: `POST {base}/bot{token}/sendMessage` with JSON
`{"chat_id": <i64|String>, "text": "...", "parse_mode": "HTML"}`.
Response is `{"ok": true, "result": {...}}` on success or
`{"ok": false, "description": "..."}` on failure — the latter is
surfaced as `Error::Api(description)`.

## Public API

- `pub fn with_config` — Build a channel from explicit base URL + bot token + chat id.
- `pub fn from_env` — Build a channel from env vars.
- Render `chat_id` as JSON: numeric `i64` if it parses, else `String`

## Related

- parent: `kei-notify-telegram/Cargo.toml`
- imports: crate, kei_runtime_core, serde, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
