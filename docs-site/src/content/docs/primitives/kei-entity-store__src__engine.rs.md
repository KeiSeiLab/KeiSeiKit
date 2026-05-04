---
title: engine.rs
path: kei-entity-store/src/engine.rs
dna_hash: sha256:8d9e1de9c1a83381
language: rust
size_loc: 124
generated: by-keidocs
---

# kei-entity-store/src/engine.rs

Store — thin wrapper over `rusqlite::Connection` that runs the
schemas' migration DDL on open.

The engine does NOT take ownership of verb dispatch. Sibling crates
call verb modules directly (e.g. `verbs::create::run(&mut conn,
&SCHEMA, input)`). This keeps the engine a passive provider of
connection + schema-aware DDL.

As of the multi-schema breaking change (2026-04-23), `Store::open`
accepts a SLICE of `&EntitySchema`. Every schema's DDL runs inside
a SINGLE transaction — if schema[i] migration fails, schema[0..i]
rolls back too. Verbs remain per-schema-dispatched by the caller.

## Public API

- `pub const CURRENT_USER_VERSION` — Schema-level version stamped into SQLite's `user_version` pragma on
- `pub fn open` — Open (creates parent dirs, enables WAL, runs migrations for all
- `pub fn open_memory` — In-memory store — unit-test constructor. Same migrations, no FS.
- `pub fn into_conn` — Escape hatch: consume the Store and return the raw Connection.
- `pub fn run_migrations` — Run all schemas' migrations atomically. For each schema: primary
- Apply one schema's DDL set inside an already-open transaction.
- Set `PRAGMA user_version` exactly once per DB lifetime (fresh DBs

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: anyhow, crate, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
