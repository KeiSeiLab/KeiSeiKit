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
