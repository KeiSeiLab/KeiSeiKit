---
title: message.rs
path: kei-gateway/src/message.rs
dna_hash: sha256:fd96d92c08256983
language: rust
size_loc: 150
generated: by-keidocs
---

# kei-gateway/src/message.rs

Normalised message event types (port of Hermes `gateway/platforms/base.py:831-908`).

Every adapter produces [`MessageEvent`]; every consumer reads it. This is the
single contract between platform-specific I/O and the agent runner.

## Public API

- Supported messaging platforms.
- Catch-all for embedded / webhook adapters that don't have a first-class enum.
- `pub fn as_str` — Stable string token used in session keys (mirrors Hermes `Platform.value`).
- Whether a chat is a 1-1 DM, a group, or a channel.
- Where a message came from. Drives session-key derivation.
- The parent chat / room / channel ID. Optional for fallback DMs.
- The user who sent the message (group isolation key).
- Alternate user ID — e.g. WhatsApp LID-vs-JID flip. Takes precedence.
- Thread / topic / reply-tree ID (Telegram forum topic, Discord thread, Slack thread).
- `pub fn dm` — Build a DM source on a platform with a single participant identifier.
- Type of inbound message content.
- A normalised inbound message event. All adapters produce this shape.
- Platform-native message ID (for replies, audit, /restart bookkeeping).
- Local file paths to downloaded media (for vision tools).
- Parallel array describing each `media_urls[i]` MIME / extension hint.
- Message ID this message replies to (for context injection).
- Per-channel ephemeral system prompt (Discord channel_prompts equivalent).
- Internal flag — synthetic events bypass user-authorisation checks.
- `pub fn is_command` — True if `text` starts with `/` (slash command convention).

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: chrono, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
