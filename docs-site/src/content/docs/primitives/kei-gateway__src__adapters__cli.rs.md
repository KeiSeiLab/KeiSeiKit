---
title: cli.rs
path: kei-gateway/src/adapters/cli.rs
dna_hash: sha256:6e819d6811958393
language: rust
size_loc: 90
generated: by-keidocs
---

# kei-gateway/src/adapters/cli.rs

CLI adapter — stdin/stdout async loop.

Reads one line at a time from stdin, wraps it in a [`MessageEvent`] under
[`Platform::Cli`], pushes it onto the inbound channel. Outbound messages
print to stdout, prefixed with `>>>` for visual separation.

This is the only fully-wired adapter in P4.1 MVP. Telegram / Discord /
Slack are stubs (see siblings).

## Public API

- `pub struct CliAdapter` — Tunables for the CLI adapter.
- Logical chat_id used in inbound events (defaults to "stdin").
- Synchronises stdout writes so concurrent sends don't interleave bytes.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: anyhow, async_trait, crate, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
