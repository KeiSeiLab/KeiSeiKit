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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
