---
title: channel.rs
path: kei-notify-slack/src/channel.rs
dna_hash: sha256:e204b8867d12b714
language: rust
size_loc: 107
generated: by-keidocs
---

# kei-notify-slack/src/channel.rs

[`SlackChannel`] — DNA-bearing [`NotifyChannel`] backed by a Slack
incoming webhook.

Constructors:
- [`SlackChannel::from_env`]: reads `SLACK_WEBHOOK_URL` from the env.
- [`SlackChannel::with_url`]: takes an explicit URL (used by tests).

`send` POSTs the [`build_payload`] JSON to the configured webhook and
treats any non-200 status as [`Error::Api`].

## Public API

- `pub const ENV_WEBHOOK_URL` — Env var holding the Slack incoming-webhook URL.
- `pub const DEFAULT_TIMEOUT_SECS` — Per-request timeout. Slack webhooks normally answer in <1s.
- Slack incoming-webhook NotifyChannel.
- `pub fn from_env` — Build a fresh channel using the URL from `SLACK_WEBHOOK_URL`.
- `pub fn with_url` — Build a channel against an explicit webhook URL (for `wiremock`).
- `pub fn webhook_url` — Direct webhook accessor (read-only; useful for assertions in tests).
- Crate-local send returning the local error (for tests that want

## Related

- parent: `kei-notify-slack/Cargo.toml`
- imports: async_trait, crate, kei_runtime_core, reqwest, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
