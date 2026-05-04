---
title: mod.rs
path: kei-gateway/src/adapters/telegram/mod.rs
dna_hash: sha256:bd665118819e059b
language: rust
size_loc: 191
generated: by-keidocs
---

# kei-gateway/src/adapters/telegram/mod.rs

Telegram adapter (P4.1.b — real teloxide impl).

Long-poll only (no webhook). `connect()` calls `getMe` for sanity. `send()`
issues `sendMessage`. `recv_loop()` consumes `update_listeners::polling_default`
and pushes deduped, normalised [`MessageEvent`]s onto the inbound channel.

Constructor pattern: this `mod.rs` orchestrates only. Conversion lives in
[`convert`], dedup in [`dedup`].

## Public API

- `pub struct TelegramAdapter` — Telegram bot adapter.
- Optional allow-list of chat IDs (string form). Empty = allow all.
- `pub fn new` — Construct an adapter from a bot token (typically read from
- `pub fn with_allowed_chats` — Restrict inbound to a specific allow-list. Builder-style.
- Drive the listener stream, dedupe, then forward to the gateway runner.
- Apply allow-list + dedup, returning a [`MessageEvent`] only if both pass.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: anyhow, async_trait, crate, futures, std, teloxide, tokio

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
