---
title: tracker.rs
path: kei-migrate/src/tracker.rs
dna_hash: sha256:b89984c912e7fcc2
language: rust
size_loc: 86
generated: by-keidocs
---

# kei-migrate/src/tracker.rs

`_kei_migrations` tracking table operations.

Row shape: (version, name, checksum, applied_at). Checksum guards against
silent edits of an applied file — mismatch = hard fail, requires human ack.

## Public API

- Create tracker table if missing. Idempotent.
- Versions of all applied migrations, ASC.
- Checksum of a specific applied version, or `None` if not applied.
- Insert a tracker row after a successful up-migration.
- Delete a tracker row after a successful down-migration.
- Abort if any applied migration's recorded checksum doesn't match the on-disk file.

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: anyhow, crate, sqlx

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
