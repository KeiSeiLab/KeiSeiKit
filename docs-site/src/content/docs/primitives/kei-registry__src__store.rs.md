---
title: store.rs
path: kei-registry/src/store.rs
dna_hash: sha256:20c372368caa4b02
language: rust
size_loc: 139
generated: by-keidocs
---

# kei-registry/src/store.rs

SQLite store — schema + open + migrate.

Constructor Pattern: this cube owns the DDL, the schema-version pragma,
and `open_db`. CRUD lives in `registry.rs`. Schema changes MUST bump
`SCHEMA_VERSION` and append to `MIGRATIONS`; never reorder.

## Public API

- `pub const SCHEMA_V1` — v1 — initial schema. Tracks one row per (path, body_sha) tuple. The DNA
- `pub const SCHEMA_V2` — v2 — formula 4-tuple columns on `blocks`. Nullable; no row rewrite needed
- `pub const SCHEMA_V3` — v3 — predicates as separate rows (1:N from blocks). One block carries
- `pub const SCHEMA_V4` — v4 — declared deps as separate rows (M:N). `dep_dna` is the wire-format
- `pub const SCHEMA_V5` — v5 — cleanup_findings table for the `kei-cleanup --emit-predicates`
- `pub const SCHEMA_VERSION` — Schema version. Compared against `PRAGMA user_version`. Bumped together
- `pub const MIGRATIONS` — Ordered migrations. Index = target version (1-based). Append only.
- `pub fn open_db` — Open or create the SQLite store at `path`. Runs all pending migrations
- `pub fn migrate` — Apply pending migrations atomically — DDL + user_version bump in one

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
