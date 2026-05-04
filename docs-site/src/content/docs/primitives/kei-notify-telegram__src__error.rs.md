---
title: error.rs
path: kei-notify-telegram/src/error.rs
dna_hash: sha256:9d9dbf2b2b0d6e59
language: rust
size_loc: 57
generated: by-keidocs
---

# kei-notify-telegram/src/error.rs

Crate-local error. Bridges into `kei_runtime_core::Error` so
`NotifyChannel::send` impls can `?` through this and surface a
substrate-level error to callers without leaking transport details.

## Public API

- Bridge into substrate-level error. Telegram-specific failures are

## Related

- parent: `kei-notify-telegram/Cargo.toml`
- imports: thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
