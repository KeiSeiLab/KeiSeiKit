---
title: store.rs
path: kei-memory-postgres/src/store.rs
dna_hash: sha256:949da654a7903a3b
language: rust
size_loc: 79
generated: by-keidocs
---

# kei-memory-postgres/src/store.rs

Connection wrapper. `tokio_postgres::connect` returns `(Client,
Connection)`; the Connection future must be polled by an executor
task, otherwise the Client deadlocks. We spawn it on the current
tokio runtime as part of [`PgStore::connect`].

## Public API

- `pub struct PgStore` — Owns the live `tokio_postgres::Client`. Cheap to wrap in `Arc` and
- Connect to PostgreSQL using a libpq-style connection string and
- Bootstrap the schema. Idempotent.
- Borrow the underlying client. Used by `PostgresBackend`; not
- `pub fn looks_like_pg_url` — Lightweight validation: a libpq URI must start with `postgres://` or

## Related

- parent: `kei-memory-postgres/Cargo.toml`
- imports: crate, tokio_postgres

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
