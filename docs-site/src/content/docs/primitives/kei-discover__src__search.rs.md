---
title: search.rs
path: kei-discover/src/search.rs
dna_hash: sha256:54c85648a2b39b3c
language: rust
size_loc: 25
generated: by-keidocs
---

# kei-discover/src/search.rs

`search` — FTS5 match over `slug` + `description`.

Thin wrapper around `kei_entity_store::verbs::search` that decodes
the JSON `results` array into typed `Entry` values. An empty query
is rejected with `InvalidInput` before dispatch (the engine enforces
the same rule but we surface the typed variant eagerly).

## Related

- parent: `kei-discover/Cargo.toml`
- imports: crate, kei_entity_store, rusqlite, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
