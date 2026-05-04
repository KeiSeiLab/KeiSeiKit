---
title: base.rs
path: kei-gateway/src/adapters/base.rs
dna_hash: sha256:d41f64bf76e298bc
language: rust
size_loc: 103
generated: by-keidocs
---

# kei-gateway/src/adapters/base.rs

[`PlatformAdapter`] trait — the contract every messaging adapter implements.

Hermes equivalent: `gateway/platforms/base.py:BasePlatformAdapter`.

## Public API

- Outbound message envelope. Adapters serialise this onto their wire format.
- Local file paths to attach (vision / voice / document delivery).
- `pub fn with_target` — Bind concrete `chat_id` / `thread_id` (called by the router).
- Adapter delivery result.
- `pub fn local` — Synthetic success for `DeliveryTarget::Local` (file-only routes).
- The trait every messaging-platform adapter implements.
- Stable platform identifier.
- One-time setup: open sockets, log in, fetch credentials.
- Send `msg` over the wire. Returns delivery confirmation or error.
- Long-running receive loop. Each inbound message becomes a

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: anyhow, async_trait, chrono, crate, serde, tokio

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
