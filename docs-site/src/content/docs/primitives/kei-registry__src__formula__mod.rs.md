---
title: mod.rs
path: kei-registry/src/formula/mod.rs
dna_hash: sha256:d11730b47783dd42
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-registry/src/formula/mod.rs

Block formula 4-tuple per `arch/MATH-DNA-DESIGN.md` §1.1.

Public API:
* [`register_formula`] — atomically write a `BlockFormula` to a row in
`blocks` and replace its rows in `block_predicates` + `block_deps`.
* [`load_formula`] — reverse-trip a row back into a `BlockFormula`,
returning `Ok(None)` when no formula has been registered.

Constructor Pattern: this module is the public surface; the type
catalogue lives in [`types`], the SHA function in [`sha`], and the SQL
plumbing in [`io`]. Each helper is one cube with one responsibility.

## Public API

- `pub fn register_formula` — Register a formula for a block. Wraps the three writes in a transaction
- `pub fn load_formula` — Load the formula for a block. Returns `Ok(None)` when the block exists

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
