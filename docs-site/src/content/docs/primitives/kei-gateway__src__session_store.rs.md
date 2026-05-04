---
title: session_store.rs
path: kei-gateway/src/session_store.rs
dna_hash: sha256:fb46cae601de534c
language: rust
size_loc: 160
generated: by-keidocs
---

# kei-gateway/src/session_store.rs

Session persistence layer (port of Hermes `gateway/session.py:640-721`).

SQLite-backed `(session_key → SessionData)` index with an in-memory LRU
cache for the hot set. Uses `sqlx` so the API is async-friendly.

## Public API

- Persistent session record.
- Opaque agent / transcript ID. The runner uses it to look up an
- Number of turns processed so far (heartbeat).
- Async session store with embedded LRU cache.
- Open or create a SQLite-backed session store.
- Look up an existing session or insert a fresh row keyed on `session_key`.
- Increment turn_count + bump updated_at. Cheap read-modify-write.
- Drop sessions whose `updated_at` is older than `cutoff`. Returns count.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: anyhow, chrono, lru, serde, sqlx, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
