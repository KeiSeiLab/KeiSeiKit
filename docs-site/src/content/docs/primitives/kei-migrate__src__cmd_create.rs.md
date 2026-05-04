---
title: cmd_create.rs
path: kei-migrate/src/cmd_create.rs
dna_hash: sha256:33559c4d1e22ff6a
language: rust
size_loc: 44
generated: by-keidocs
---

# kei-migrate/src/cmd_create.rs

`kei-migrate create <name>` — scaffold a new timestamped migration pair.

## Public API

- `pub fn run` — Create `<dir>/<utc-timestamp>_<sanitized-name>.sql` + `.down.sql`. Returns paths written.

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: anyhow, chrono, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
