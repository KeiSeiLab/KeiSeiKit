---
title: lib.rs
path: kei-migrate/src/lib.rs
dna_hash: sha256:e7c0dba52271ef63
language: rust
size_loc: 54
generated: by-keidocs
---

# kei-migrate/src/lib.rs

kei-migrate — universal SQL migration runner.

Single binary, three backends (Postgres / SQLite / MySQL) autodetected
from `DATABASE_URL`. Sequential `.sql` files in `migrations/`, tracked in
`_kei_migrations` with SHA-256 checksums.

Library surface exists so integration tests can drive the primitive
without `process::Command` gymnastics.

## Public API

- End-to-end `up` entry: connect, ensure tracker, scan dir, apply pending.
- End-to-end `down` entry: revert last N applied.
- End-to-end `status` entry: returns (applied, pending) counts.

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: anyhow, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
