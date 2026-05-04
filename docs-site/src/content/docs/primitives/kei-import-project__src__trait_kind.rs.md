---
title: trait_kind.rs
path: kei-import-project/src/trait_kind.rs
dna_hash: sha256:173eae98f5bfa557
language: rust
size_loc: 77
generated: by-keidocs
---

# kei-import-project/src/trait_kind.rs

Trait-kind parsing + enumeration helpers.

Extracted from `trait_patterns.rs` to keep that file under the
Constructor Pattern 200-LOC ceiling. The 12 patterns + their
supporting structs live in `trait_patterns.rs`; this module owns
the case-insensitive `from_str_ci` parser and the `all()`
enumeration helper.

## Public API

- `pub fn from_str_ci` — Parse a case-insensitive kebab-case name into a `TraitKind`.
- `pub fn all` — All 12 trait kinds in definition order.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
