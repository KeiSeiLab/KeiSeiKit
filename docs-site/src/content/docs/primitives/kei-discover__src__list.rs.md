---
title: list.rs
path: kei-discover/src/list.rs
dna_hash: sha256:b97e67006086a889
language: rust
size_loc: 41
generated: by-keidocs
---

# kei-discover/src/list.rs

`list_available` — return entries that have NOT been installed.

Runs a direct `SELECT` rather than `kei_entity_store::verbs::list`
because the generic verb does not filter by column; we need
`WHERE installed = 0` to hide already-installed entries. Ordering is
`id DESC` for consistency with the engine's `list` convention.

## Related

- parent: `kei-discover/Cargo.toml`
- imports: crate, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
