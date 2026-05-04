---
title: mod.rs
path: kei-gateway/src/adapters/slack/mod.rs
dna_hash: sha256:f266538fb2ca1e48
language: rust
size_loc: 175
generated: by-keidocs
---

# kei-gateway/src/adapters/slack/mod.rs

Slack adapter (P6 — real slack-morphism impl).

`connect()` = api.test. `send()` = chat.postMessage. `recv_loop()` =
Socket Mode WebSocket via apps.connections.open. Submodules: `convert`,
`dedup`.

## Public API

- `pub struct SlackAdapter` — Slack bot adapter. `bot_token` authenticates REST calls; `app_token`
- `pub fn new` — Construct from a bot token (read from env per RULE 0.8 — never hardcode).
- `pub fn with_app_token` — Enable Socket Mode for `recv_loop`. `app_token` must start with `xapp-`.
- `pub fn with_allowed_channels` — Restrict inbound to specific channel IDs. Builder-style.
- Apply allow-list and dedup. Returns `Some(event)` only when both pass.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: anyhow, async_trait, crate, futures, slack_morphism, std, tokio, tokio_tungstenite

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
