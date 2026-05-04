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
