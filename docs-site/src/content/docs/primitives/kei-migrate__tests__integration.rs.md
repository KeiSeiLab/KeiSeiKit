---
title: integration.rs
path: kei-migrate/tests/integration.rs
dna_hash: sha256:3110226e9fa9ddc0
language: rust
size_loc: 174
generated: by-keidocs
---

# kei-migrate/tests/integration.rs

Integration tests for kei-migrate against a SQLite file (safe, no deps).

SQLite is chosen as the test backend because it has no server dependency
and the sqlx-Any path through it exercises the same code path as Postgres
/ MySQL for everything except dialect-specific DDL (which we abstract in
`db::Backend::create_tracker_sql`).

## Related

- parent: `kei-migrate/tests`
- imports: kei_migrate, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
