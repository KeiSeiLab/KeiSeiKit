---
title: mod.rs
path: kei-skill-importer/src/emit/mod.rs
dna_hash: sha256:865819d8b4cebb14
language: rust
size_loc: 69
generated: by-keidocs
---

# kei-skill-importer/src/emit/mod.rs

Emit path — decide WHICH canonical KeiSeiKit shape to render the
imported skill into, then dispatch to the matching emitter.

## Public API

- Three target shapes the importer can emit into.
- Standalone atom — `_primitives/_rust/<crate>/atoms/<verb>.md`.
- Recipe DAG — `recipes/<name>.toml`.
- Proposed primitive — `_primitives/proposed/<name>.md`.
- `pub fn decide_emit_path` — Decision logic — chooses an emit path based on detected atom-call

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
