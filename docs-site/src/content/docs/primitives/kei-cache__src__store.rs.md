---
title: store.rs
path: kei-cache/src/store.rs
dna_hash: sha256:3af8fe6ea72e127f
language: rust
size_loc: 237
generated: by-keidocs
---

# kei-cache/src/store.rs

SQLite-backed cache store.

Constructor Pattern: one cube = cache table DDL + put/get/stats/purge/clear.
Every fn <30 LOC. Schema is append-only migration list; expiry is
timestamp-based (`expires_ts = created_ts + ttl_sec`).

Layout: one row per unique (atom_id, canonical_input) → cache key.
Payload stored as raw JSON text to keep the primitive format-neutral.

## Public API

- `pub const MIGRATIONS` — Ordered migrations. Index = schema version. Never reorder; append only.
- `pub fn open` — Open or create the cache DB and run migrations.
- Apply pending migrations atomically (DDL + user_version bump per txn).
- `pub fn now_ts` — Current unix timestamp in seconds.
- `pub fn put` — Insert (upsert) a cache entry. `ttl_sec` must be > 0.
- `pub fn get` — Look up a key; returns `None` on miss or expired entry.
- `pub fn bump` — Increment a named counter (hits / misses) by 1.
- `pub fn stats` — Read aggregate stats: (hits, misses, live_entries, total_bytes).
- Aggregate cache stats snapshot.
- `pub fn purge` — Evict expired rows; returns number deleted.
- `pub fn clear` — Wipe everything (cache + counters). Returns rows removed from `cache`.

## Related

- parent: `kei-cache/Cargo.toml`
- imports: anyhow, rusqlite, std, tempfile

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
