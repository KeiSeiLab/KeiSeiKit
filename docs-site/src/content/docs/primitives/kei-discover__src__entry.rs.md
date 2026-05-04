---
title: entry.rs
path: kei-discover/src/entry.rs
dna_hash: sha256:d3e4d128add5dc23
language: rust
size_loc: 46
generated: by-keidocs
---

# kei-discover/src/entry.rs

`Entry` — typed view of one `discover_index` row returned by
`list_available` / `search`.

Conversion from the engine's `serde_json::Value` row is centralised
here so every public API function returns the same typed struct.

## Public API

- `pub fn from_json` — Decode one row produced by `kei_entity_store::verbs::{get,list,search}`.

## Related

- parent: `kei-discover/Cargo.toml`
- imports: serde, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
