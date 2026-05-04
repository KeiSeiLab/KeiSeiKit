---
title: sqlite_store.rs
path: kei-ping/src/sqlite_store.rs
dna_hash: sha256:bdd2f314e989edb3
language: rust
size_loc: 121
generated: by-keidocs
---

# kei-ping/src/sqlite_store.rs

SQLite-backed PingStore. WAL + busy_timeout for concurrent windows.
1 row per agent_id; UPDATE on every heartbeat (idempotent).

## Related

- parent: `kei-ping/Cargo.toml`
- imports: anyhow, crate, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
