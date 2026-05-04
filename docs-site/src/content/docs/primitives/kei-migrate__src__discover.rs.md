---
title: discover.rs
path: kei-migrate/src/discover.rs
dna_hash: sha256:d39de139fbd7bcaa
language: rust
size_loc: 87
generated: by-keidocs
---

# kei-migrate/src/discover.rs

Filesystem migration discovery.

Convention: `migrations/<version>_<name>.sql` (up) and optional
`migrations/<version>_<name>.down.sql` (down). Version is a monotonic
integer, typically a UTC timestamp like `20260421120000`.

## Public API

- One discovered migration (up-side). `down_path` is `Some` iff the sibling file exists.
- `pub fn scan` — Read every `<version>_<name>.sql` file (ignoring `.down.sql`), sort by version ASC.

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: anyhow, sha2, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
