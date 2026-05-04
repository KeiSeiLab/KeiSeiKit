---
title: backend.rs
path: kei-memory-redis/src/backend.rs
dna_hash: sha256:d73155f1e4aafadc
language: rust
size_loc: 235
generated: by-keidocs
---

# kei-memory-redis/src/backend.rs

[`RedisBackend`] — `MemoryBackend` impl over [`crate::RedisStore`].

Storage layout (see `store.rs`):
- `<prefix>:item:<kind>:<ts>:<key>` → JSON-serialized [`MemoryItem`]
- `<prefix>:tag:<tag>`              → SET of item-id strings

`compact(since_ms)` deletes items strictly older than `since_ms`
(i.e. `parsed.ts_ms < since_ms`) and returns the deleted count.
Tag-set entries pointing at deleted items are removed in the same
pass to keep query-by-tag honest.

`mirror_to_remote` is intentionally unimplemented: cross-Redis
replication is the operator's responsibility (Redis replication /
AOF), not this primitive's. Returns `Provider`.

## Public API

- SCAN every `<prefix>:item:<kind?>:*` key and collect them. Used

## Related

- parent: `kei-memory-redis/Cargo.toml`
- imports: crate, kei_runtime_core, redis

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
