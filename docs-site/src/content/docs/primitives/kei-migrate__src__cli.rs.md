---
title: cli.rs
path: kei-migrate/src/cli.rs
dna_hash: sha256:7ccfccc904b3aa1e
language: rust
size_loc: 44
generated: by-keidocs
---

# kei-migrate/src/cli.rs

CLI surface — clap argument parsing for `kei-migrate`.

## Public API

- Database URL. Overrides $DATABASE_URL.
- Migrations directory (default: ./migrations)
- Apply all pending migrations.
- Revert the last N migrations (requires <ts>_<name>.down.sql).
- List applied vs pending migrations.
- Create a new timestamped migration scaffold: <ts>_<name>.sql (+ .down.sql).
- Short migration name, e.g. "add_users_email_index".

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: clap

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
