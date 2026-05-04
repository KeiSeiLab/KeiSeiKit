---
title: main.rs
path: kei-projects-index/src/main.rs
dna_hash: sha256:81926c534f402c1e
language: rust
size_loc: 56
generated: by-keidocs
---

# kei-projects-index/src/main.rs

kei-projects-index — CLI dispatcher.

Constructor Pattern: this binary holds clap shapes only. Every command
forwards to a function in the sibling `cli` / `query` cubes or to
`kei_projects_index::*` library APIs.

## Public API

- Create the DB file and apply the schema.
- Walk projects_root and refresh every row.
- Print all rows as a JSON array to stdout.
- Print one project row as JSON.

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: clap, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
