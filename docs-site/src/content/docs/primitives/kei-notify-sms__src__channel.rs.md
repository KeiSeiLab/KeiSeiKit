---
title: channel.rs
path: kei-notify-sms/src/channel.rs
dna_hash: sha256:2950b0c45d13792f
language: rust
size_loc: 165
generated: by-keidocs
---

# kei-notify-sms/src/channel.rs

[`SmsChannel`] — `NotifyChannel` impl for Twilio Programmable Messaging.

Hits exactly one endpoint:
`POST /2010-04-01/Accounts/{ACCOUNT_SID}/Messages.json` with form body
`To=...&From=...&Body=...` and HTTP Basic auth
(`ACCOUNT_SID:AUTH_TOKEN`). Twilio answers 201 Created with a JSON
payload containing the message `sid` on success and a `{code, message}`
pair on 4xx.

## Public API

- `pub struct SmsChannel` — Twilio Programmable Messaging SMS channel.
- Twilio 4xx error envelope. Both fields are always present in 4xx/5xx
- `pub fn from_env` — Build from process env: `TWILIO_ACCOUNT_SID`, `TWILIO_AUTH_TOKEN`,
- `pub fn with_config` — Explicit-config constructor. `base_url` lets `wiremock` tests
- SMS is intrusive and metered. Drop anything below `Warn` by

## Related

- parent: `kei-notify-sms/Cargo.toml`
- imports: async_trait, crate, kei_runtime_core, reqwest, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
