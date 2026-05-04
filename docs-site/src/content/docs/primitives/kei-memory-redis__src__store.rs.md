---
title: store.rs
path: kei-memory-redis/src/store.rs
dna_hash: sha256:40db6764a562d8f4
language: rust
size_loc: 156
generated: by-keidocs
---

# kei-memory-redis/src/store.rs

Thin wrapper over `redis::Client` plus the deterministic key-schema
used by [`crate::backend::RedisBackend`]. Holds no trait impls so the
schema helpers can be unit-tested without a live Redis.

Schema (deterministic; documented in spec/MEMORY-BACKENDS.md §Redis):

```text
<prefix>:item:<kind>:<created_at_ms>:<key>   → JSON-encoded MemoryItem
<prefix>:tag:<tag>                            → SET of item-ids
```

`item-id` is the encoded item key string above (the full path); this
lets a tag-driven query resolve straight to the JSON GET without an
extra index hop.

## Public API

- `pub struct RedisStore` — Redis client + scope prefix. Connections are short-lived: every call
- `pub fn from_url` — Connect by URL (`redis://host:port`, `rediss://...`, etc).
- Hand out a fresh multiplexed async connection per call. The
- `pub fn item_match` — SCAN match-pattern filtered by optional kind. `*` is used as a
- `pub fn encode_item_key` — Compose `<prefix>:item:<kind>:<ts>:<key>`.
- `pub fn encode_tag_key` — Compose `<prefix>:tag:<tag>`.
- Parsed view of an `item` key. None on malformed input.
- `pub fn decode_item_key` — Inverse of [`encode_item_key`]. Returns `None` if the input does not

## Related

- parent: `kei-memory-redis/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
