---
title: channel.rs
path: kei-notify-discord/src/channel.rs
dna_hash: sha256:60db83ff1bd8f1c0
language: rust
size_loc: 94
generated: by-keidocs
---

# kei-notify-discord/src/channel.rs

[`DiscordChannel`] — `NotifyChannel` impl backed by a Discord webhook.

`channel_name = "discord"`. `supports_batching = false` — Discord
webhooks accept one message per POST (no native digest). DNA carries
caps `["PR", "AP", "DC"]` per the Wave 8 atomar branding axes.

## Public API

- `pub struct DiscordChannel` — Discord webhook NotifyChannel.
- `pub fn from_env` — Build from the `DISCORD_WEBHOOK_URL` env var (returned wrapped in
- `pub fn with_url` — Explicit-URL constructor — the wiremock test path.
- `pub fn webhook_url` — Borrow the configured webhook URL.

## Related

- parent: `kei-notify-discord/Cargo.toml`
- imports: async_trait, crate, kei_runtime_core, reqwest, std

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
