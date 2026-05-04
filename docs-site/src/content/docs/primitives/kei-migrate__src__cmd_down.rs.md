---
title: cmd_down.rs
path: kei-migrate/src/cmd_down.rs
dna_hash: sha256:7635ad63aa64b955
language: rust
size_loc: 58
generated: by-keidocs
---

# kei-migrate/src/cmd_down.rs

`kei-migrate down [n]` — revert the last N applied migrations.

Requires a sibling `<version>_<name>.down.sql` for each target. Missing
down-file = hard error — we don't guess reversals.

## Public API

- Revert the last `n` applied migrations in reverse order.

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: anyhow, crate, sqlx, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
