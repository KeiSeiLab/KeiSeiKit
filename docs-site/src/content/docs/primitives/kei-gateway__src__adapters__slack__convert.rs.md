---
title: convert.rs
path: kei-gateway/src/adapters/slack/convert.rs
dna_hash: sha256:81a236526b4519ad
language: rust
size_loc: 137
generated: by-keidocs
---

# kei-gateway/src/adapters/slack/convert.rs

Pure conversion: Slack Socket Mode `message` event payload → gateway
[`MessageEvent`].

Kept free of network calls so the conversion can be unit-tested by feeding
deserialised JSON fixtures (no live Slack API).

## Public API

- Subset of the Slack `event_callback` envelope we care about.
- Inner event — we only handle `message` subtypes; anything else is ignored.
- Channel ID where the message was posted.
- Unique message timestamp (also serves as message ID).
- Thread parent timestamp (set for threaded replies).
- User ID of the sender. Absent for bot messages.
- Plain-text content of the message.
- Channel type: `"im"` = DM, `"channel"` = public, etc.
- `pub fn event_to_message` — Convert a raw Socket Mode event callback JSON `value` into a

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: crate, serde

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
