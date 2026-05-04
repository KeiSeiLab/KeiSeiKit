---
title: lib.rs
path: kei-comments/src/lib.rs
dna_hash: sha256:38badc568bc6bca2
language: rust
size_loc: 193
generated: by-keidocs
---

# kei-comments/src/lib.rs

kei-comments — sovereign threaded comment store for KeiWiki.
Replaces Giscus / GitHub Discussions. SQLite-backed, single-process,
soft-delete + reactions. Auth is gated upstream by cortex daemon.

## Public API

- `pub const MAX_BODY_BYTES` — Hard cap on comment body in bytes — prevents DOS via large inserts.
- `pub const SCHEMA_SQL` — Idempotent schema. Created on every `migrate()` call.
- Public Comment row. Soft-deleted rows present `deleted = true` with body wiped.
- `pub struct CommentStore` — SQLite-backed comment store.
- `pub fn open` — Open or create the database file. Caller must invoke `migrate()` once.
- `pub fn open_memory` — In-memory store, for tests.
- `pub fn migrate` — Idempotent — safe to call on every startup.
- `pub fn post` — Insert a new comment. Returns the assigned id (16-hex of sha256).
- `pub fn list` — List all (incl. soft-deleted) comments for a page, ordered by created_at.
- `pub fn get` — Fetch by id; None if not present.
- `pub fn delete` — Soft-delete: only the original author may delete. Returns true on success.
- `pub fn react` — Add a reaction (idempotent — repeat is no-op via PK conflict).
- `pub fn unreact` — Remove a reaction (idempotent — missing row is no-op).
- `pub fn reactions` — Map of emoji → list of authors who reacted with it.

## Related

- parent: `kei-comments/Cargo.toml`
- imports: anyhow, chrono, rusqlite, serde, sha2, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
