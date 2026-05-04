---
title: store.rs
path: kei-memory-sled/src/store.rs
dna_hash: sha256:c8bebe802d29731b
language: rust
size_loc: 119
generated: by-keidocs
---

# kei-memory-sled/src/store.rs

Sled-backed storage layer. The async surface lives in `backend.rs`;
this module is sync (sled is sync) and exposes blocking helpers
that the backend wraps in `tokio::task::spawn_blocking`.

Key encoding (lex-sortable for `scan_prefix` + chronological order):

```text
<kind>\x00<ts_be_8>\x00<key>
```

- `kind` bytes form the prefix (1 NUL terminator → no kind-name bleed).
- `ts_be_8` is `i64::to_be_bytes` so newer items sort *after* older.
- `key` is appended last so identical-timestamp items can coexist.

Values are JSON-serialized `MemoryItem`.

## Public API

- Owned handle around a `sled::Db`. Cheap to clone (sled::Db is Arc).
- `pub fn from_path` — Open or create a sled DB at `path`. Directory is created if absent.
- `pub fn raw` — Underlying `sled::Db` for advanced ops (flush, size, etc).
- `pub fn put_item` — Insert (or overwrite) one `MemoryItem`.
- `pub fn scan` — Scan items, optionally restricted to `kind`. Returns DESC by ts.
- `pub fn count_older_than` — Count items in `kind` strictly older than `since_ms`.
- `pub fn flush` — Force a flush to disk. Mostly for tests.
- `pub fn encode_key` — Encode `<kind>\x00<ts_be>\x00<key>` for prefix-scan + ordering.

## Related

- parent: `kei-memory-sled/Cargo.toml`
- imports: crate, kei_runtime_core, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
