---
title: dedup.rs
path: kei-gateway/src/adapters/telegram/dedup.rs
dna_hash: sha256:b6b54911549774ba
language: rust
size_loc: 95
generated: by-keidocs
---

# kei-gateway/src/adapters/telegram/dedup.rs

Inbound dedup for the Telegram adapter.

Telegram's long-poll loop occasionally re-delivers the same `Update` (network
re-tries, getUpdates offset glitches). We hash `(chat_id, message_id, text)`
with blake3 and keep a bounded LRU set of recently-seen hashes — duplicates
are silently dropped before we push a [`MessageEvent`] downstream.

## Public API

- `pub const DEFAULT_CAPACITY` — Default capacity. ~512 hashes × 32 bytes ≈ 16 KiB — fits in cache.
- `pub struct DedupCache` — Bounded "have-I-seen-this-before" set keyed on a 32-byte blake3 digest.
- `pub fn new` — Build a dedup cache holding up to `capacity` recent message hashes.
- `pub fn observe` — True iff `hash` had already been seen. Touches the LRU on hit.
- `pub fn message_digest` — Hash `(chat_id, message_id, text)` to a stable 32-byte digest.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: lru, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
