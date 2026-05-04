---
title: sha.rs
path: kei-registry/src/formula/sha.rs
dna_hash: sha256:0a4b54146101a65b
language: rust
size_loc: 42
generated: by-keidocs
---

# kei-registry/src/formula/sha.rs

Canonical short-sha8 of a `BlockFormula` (excluding `block_id`).

Constructor Pattern: hashing is one cube. The canonical serialisation
uses `serde_json` over the four formula facets in the canonical order
`(type, invariants, effects, deps)`. `effects` and `deps` are
`BTreeSet`, so their iteration order is sorted; `invariants` order is
the author's chosen order — predicates are positional by design.

## Public API

- `pub fn formula_sha` — Compute the deterministic 8-hex-char digest of the formula's contents.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: sha2

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
