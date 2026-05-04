---
title: mod.rs
path: kei-gateway/src/adapters/discord/mod.rs
dna_hash: sha256:8ea5e95c2cc14806
language: rust
size_loc: 200
generated: by-keidocs
---

# kei-gateway/src/adapters/discord/mod.rs

Discord adapter (P5 — serenity gateway impl).
Gateway-mode only. Conversion lives in [`convert`], dedup in [`dedup`].

## Public API

- `pub struct DiscordAdapter` — Discord bot adapter.
- Optional allow-list of channel IDs (string form). Empty = allow all.
- `pub fn new` — Construct an adapter from a bot token (from `~/.claude/secrets/.env`, RULE 0.8).
- `pub fn with_allowed_channels` — Restrict inbound to a channel allow-list. Builder-style.
- Send a message with an optional reply reference.
- serenity `EventHandler` that forwards text messages to the gateway runner.
- Apply allow-list + dedup, returning a [`MessageEvent`] only if both pass.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: anyhow, async_trait, crate, serenity, std, tokio

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
