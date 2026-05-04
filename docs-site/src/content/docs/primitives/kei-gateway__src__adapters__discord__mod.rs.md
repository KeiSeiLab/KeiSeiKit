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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
