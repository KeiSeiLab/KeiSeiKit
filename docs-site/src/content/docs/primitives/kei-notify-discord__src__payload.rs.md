---
title: payload.rs
path: kei-notify-discord/src/payload.rs
dna_hash: sha256:0e31d08ebd7f6fea
language: rust
size_loc: 94
generated: by-keidocs
---

# kei-notify-discord/src/payload.rs

Payload builder: `Notification` → Discord webhook JSON.

Discord webhook expects:
```json
{
"content": "...",
"embeds": [{"title": "...", "description": "...", "color": <int>}]
}
```

Color is decimal RGB (NOT hex). Mapping is fixed by severity.

## Public API

- `pub fn color_for` — Discord embed color (decimal RGB) per severity.
- `pub fn build_payload` — Build the JSON body POSTed to a Discord webhook.

## Related

- parent: `kei-notify-discord/Cargo.toml`
- imports: kei_runtime_core, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
