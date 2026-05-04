---
title: payload.rs
path: kei-notify-sms/src/payload.rs
dna_hash: sha256:3266f6c4b9e7f0a9
language: rust
size_loc: 113
generated: by-keidocs
---

# kei-notify-sms/src/payload.rs

SMS body composition: severity-emoji prefix + subject + em-dash +
body_text, then UTF-8-safe truncation to 1500 bytes (Twilio's hard
per-segment limit is 1600; we keep 100 bytes of headroom).

## Public API

- Hard byte cap. Twilio's `Body` parameter accepts up to 1600 chars; we
- `pub fn severity_emoji` — Map a [`NotifySeverity`] to a single-glyph prefix. Plain ASCII so the
- `pub fn build_body` — Compose the wire body from a `Notification`. Format:
- Truncate `s` to at most `max_bytes` bytes without splitting a UTF-8

## Related

- parent: `kei-notify-sms/Cargo.toml`
- imports: kei_runtime_core

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
