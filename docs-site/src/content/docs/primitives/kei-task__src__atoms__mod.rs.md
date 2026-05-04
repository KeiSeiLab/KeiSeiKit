---
title: mod.rs
path: kei-task/src/atoms/mod.rs
dna_hash: sha256:db6dc7ed78e0acc5
language: rust
size_loc: 54
generated: by-keidocs
---

# kei-task/src/atoms/mod.rs

kei-task atoms — one file per verb, each exposing
`pub fn run(store, input) -> Result<Output, Error>`.

Reference implementation for the substrate schema (see
`docs/SUBSTRATE-SCHEMA.md`). Every other kei-* crate will follow
this shape in v0.24+.

## Public API

- `pub const VERBS` — Verbs exposed through the `run-atom <verb>` machine-facing CLI.
- Errors from the `run-atom` dispatcher layer itself — NOT from the atom

## Related

- parent: `kei-task/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
