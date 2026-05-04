---
title: query.rs
path: kei-projects-index/src/query.rs
dna_hash: sha256:f64d3bacc1bb6173
language: rust
size_loc: 60
generated: by-keidocs
---

# kei-projects-index/src/query.rs

Read-only query helpers used by the CLI to render the `list` and
`get` outputs. The library `index::ProjectRow` shape is the SSoT.

## Public API

- `pub fn list_all` — All rows ordered most-recently-committed first (NULLS LAST). The
- `pub fn get_one` — Fetch a single row by primary key (`path`). Returns `Ok(None)` if no

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: crate, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
