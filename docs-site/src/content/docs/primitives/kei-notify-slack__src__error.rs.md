---
title: error.rs
path: kei-notify-slack/src/error.rs
dna_hash: sha256:7e4778a9d608be73
language: rust
size_loc: 44
generated: by-keidocs
---

# kei-notify-slack/src/error.rs

Local error type for the Slack notify channel.

Mapped into [`kei_runtime_core::Error`] via `From<Error>` so the trait
impls can use `?` against the runtime-core `Result`.

## Public API

- `pub type Result` — Crate-local result alias.
- Crate-local error variants.
- Transport / TLS / timeout failure from `reqwest`.
- Non-200 HTTP status with the (best-effort) body text.
- DNA construction or env-var read failure.

## Related

- parent: `kei-notify-slack/Cargo.toml`
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
