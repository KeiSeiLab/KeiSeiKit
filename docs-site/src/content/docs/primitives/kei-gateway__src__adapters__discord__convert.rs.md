---
title: convert.rs
path: kei-gateway/src/adapters/discord/convert.rs
dna_hash: sha256:049fbe6d98156673
language: rust
size_loc: 152
generated: by-keidocs
---

# kei-gateway/src/adapters/discord/convert.rs

Pure conversion: serenity `Message` → gateway [`MessageEvent`].

Kept free of network calls so the conversion can be unit-tested by
constructing `Message` values from JSON fixtures (no live Discord API).
Guild presence is inferred from `msg.guild_id` — absent means DM.

## Public API

- `pub fn message_to_event` — Convert a serenity `Message` to a [`MessageEvent`].
- DM if `guild_id` is absent; guild text channel otherwise.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: crate, serenity

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
