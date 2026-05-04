---
title: lib.rs
path: kei-notify-slack/src/lib.rs
dna_hash: sha256:09e89de15b2d1828
language: rust
size_loc: 21
generated: by-keidocs
---

# kei-notify-slack/src/lib.rs

kei-notify-slack — Slack incoming-webhook impl of
[`kei_runtime_core::NotifyChannel`].

Layout (Constructor Pattern, ≤200 LOC per file):
- [`error`]: local `Error`/`Result` mapped into runtime-core error.
- [`payload`]: pure `build_payload` function (severity → attachment colour).
- [`channel`]: [`SlackChannel`] — DNA-bearing trait impl.

Auth: webhook URL read from env `SLACK_WEBHOOK_URL`. URL is overridable
via [`SlackChannel::with_url`] for `wiremock` tests.

## Related

- parent: `kei-notify-slack/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
