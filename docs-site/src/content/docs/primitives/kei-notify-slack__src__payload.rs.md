---
title: payload.rs
path: kei-notify-slack/src/payload.rs
dna_hash: sha256:17c1ed1203989ee1
language: rust
size_loc: 111
generated: by-keidocs
---

# kei-notify-slack/src/payload.rs

Slack incoming-webhook payload builder.

Pure function — takes a [`Notification`] and emits a `serde_json::Value`
shaped for the Slack `chat.postMessage`-compatible incoming-webhook
contract:

```json
{
"text": "<subject>",
"attachments": [
{ "color": "good|warning|danger|#3b82f6", "title": "...", "text": "..." }
]
}
```

Colour mapping (Slack conventions + one CSS hex for Info):
- `Info`    → `#3b82f6`  (blue)
- `Success` → `good`     (green)
- `Warn`    → `warning`  (yellow)
- `Error`   → `danger`   (red)

## Public API

- `pub fn severity_color` — Map [`NotifySeverity`] to the Slack attachment `color` value.
- `pub fn build_payload` — Build the JSON body for a Slack incoming-webhook POST.

## Related

- parent: `kei-notify-slack/Cargo.toml`
- imports: kei_runtime_core, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
