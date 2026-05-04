---
title: types.rs
path: kei-registry/src/formula/types.rs
dna_hash: sha256:922f8c3900ab4a2d
language: rust
size_loc: 157
generated: by-keidocs
---

# kei-registry/src/formula/types.rs

Formula types — pure data shape for the block formula 4-tuple.

Constructor Pattern: this cube owns ONLY the type definitions. Hashing
lives in `sha.rs`; SQL persistence in `io.rs`; the public API in
`mod.rs`. Mirrors `arch/MATH-DNA-DESIGN.md` §1.1.

## Public API

- Block formula 4-tuple: (Type, Invariants, Effects, Deps) anchored to a
- Function-level type signature: the declared input/output/error atoms.
- Coarse type atoms recognised by the registry. `Custom` carries an
- Invariant predicate — a checkable assertion about the workspace state
- Symbol-kind tag used by the `SymbolDeclared` predicate.
- Side-effect classification. Sorted by `Ord` so the canonical hash is
- Provenance of a formula. `Inferred` carries a 0..100 confidence score
- `pub fn predicate_kind` — Stable string tag for a `Predicate` variant — used as the `kind`

## Related

- parent: `kei-registry/Cargo.toml`
- imports: serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
