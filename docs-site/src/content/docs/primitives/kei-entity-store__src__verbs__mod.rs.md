---
title: mod.rs
path: kei-entity-store/src/verbs/mod.rs
dna_hash: sha256:6b8f5c86184ca488
language: rust
size_loc: 30
generated: by-keidocs
---

# kei-entity-store/src/verbs/mod.rs

Verb templates — one module per generic CRUD / graph verb.

Each verb exposes `pub fn run(conn, schema, input) -> Result<Value,
VerbError>` with JSON in / JSON out. Sibling crates wrap these in
their typed atom `Input` / `Output` structs via `serde_json::from_value`.

The `input` arg is always a `serde_json::Value`. Verbs extract fields
they need and ignore everything else, except `create` / `update` which
only copy declared schema fields into SQL (defence against
unexpected keys).

## Public API

- `pub const ALL_VERBS` — Full list of supported verbs — SSoT for documentation + schema

## Related

- parent: `kei-entity-store/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
