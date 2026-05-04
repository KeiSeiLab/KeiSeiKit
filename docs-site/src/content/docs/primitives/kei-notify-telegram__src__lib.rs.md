---
title: lib.rs
path: kei-notify-telegram/src/lib.rs
dna_hash: sha256:ae36e929cfe57dfe
language: rust
size_loc: 29
generated: by-keidocs
---

# kei-notify-telegram/src/lib.rs

kei-notify-telegram — `NotifyChannel` impl over the Telegram Bot API.

Endpoint: `POST {base}/bot{TOKEN}/sendMessage` with JSON body
`{"chat_id": <i64 or "@channel">, "text": "...", "parse_mode": "HTML"}`.
Severity is rendered as a leading emoji in the body so a single
channel-route surfaces all four `NotifySeverity` levels visually.

Constructor Pattern: 3 source files, each <200 LOC, one
responsibility per cube:
* `error.rs`   — error type + `From<reqwest::Error>` +
bridge into `kei_runtime_core::Error::Provider`
* `payload.rs` — body composition (severity emoji + HTML escape)
* `channel.rs` — `TelegramChannel` (env + with_config constructors,
`NotifyChannel::send` over reqwest)

Auth: bot token from `TELEGRAM_BOT_TOKEN`, target from
`TELEGRAM_CHAT_ID` (numeric `i64` or `@channel_username`). Base URL
defaults to `https://api.telegram.org` and is overridable via
constructor so wiremock tests can point at a localhost mock.

## Related

- parent: `kei-notify-telegram/Cargo.toml`

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
