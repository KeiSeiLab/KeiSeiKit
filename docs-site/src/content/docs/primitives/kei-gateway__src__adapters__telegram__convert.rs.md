---
title: convert.rs
path: kei-gateway/src/adapters/telegram/convert.rs
dna_hash: sha256:4541706e89324660
language: rust
size_loc: 102
generated: by-keidocs
---

# kei-gateway/src/adapters/telegram/convert.rs

Pure conversion: teloxide `Update` → gateway [`MessageEvent`].

Kept free of network calls so the conversion can be unit-tested by feeding
deserialised JSON fixtures (no live Telegram API).

## Public API

- `pub fn update_to_event` — Result of converting an `Update`. `None` = update was not a text message we
- JSON fixture of a private (DM) text message.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: crate, teloxide

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
