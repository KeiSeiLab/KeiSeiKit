---
title: cli.rs
path: kei-projects-index/src/cli.rs
dna_hash: sha256:96d07e5d16addfe8
language: rust
size_loc: 95
generated: by-keidocs
---

# kei-projects-index/src/cli.rs

CLI command handlers — keeps `main.rs` ≤ 30 LOC by holding the four
sub-command implementations as one cube.

## Public API

- `pub fn cmd_init` — `init` — open / create DB and apply schema.
- `pub fn cmd_rebuild` — `rebuild` — walk root and refresh all rows.
- `pub fn cmd_list` — `list` — print all rows as JSON.
- `pub fn cmd_get` — `get` — print one row by path.

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: kei_projects_index, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
