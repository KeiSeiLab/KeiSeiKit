---
title: dedup.rs
path: kei-gateway/src/adapters/slack/dedup.rs
dna_hash: sha256:2c93d568ac2a71d7
language: rust
size_loc: 100
generated: by-keidocs
---

# kei-gateway/src/adapters/slack/dedup.rs

Inbound dedup for the Slack adapter.

Slack Socket Mode occasionally re-delivers events. We hash
`(channel_id, ts, text)` with blake3 and keep a bounded LRU set of
recently-seen hashes — duplicates are silently dropped before we push a
[`crate::message::MessageEvent`] downstream.

## Public API

- `pub const DEFAULT_CAPACITY` — Default capacity. ~512 hashes × 32 bytes ≈ 16 KiB — fits in cache.
- `pub struct DedupCache` — Bounded "have-I-seen-this-before" set keyed on a 32-byte blake3 digest.
- `pub fn new` — Build a dedup cache holding up to `capacity` recent message hashes.
- `pub fn observe` — True iff `hash` had already been seen. Touches the LRU on hit.
- `pub fn message_digest` — Hash `(channel_id, ts, text)` to a stable 32-byte digest.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: lru, std

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
