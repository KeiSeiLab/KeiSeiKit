---
title: walker.rs
path: kei-arch-derive/src/walker.rs
dna_hash: sha256:d271d43671eb6dab
language: rust
size_loc: 121
generated: by-keidocs
---

# kei-arch-derive/src/walker.rs

Walk the workspace for `[package.metadata.keisei.formula]` declarations
in member `Cargo.toml` files.

Each declaration yields a `FormulaDecl`: the crate path (relative to
workspace root), the package name, the parsed effects list, and the
list of declared invariants. Used by `emit::derive_plan` to bridge a
kei-registry-driven derivation with hand-declared formulas.

Constructor Pattern: this cube ONLY does discovery + parsing. No
projection, no emission. Returns sorted, deterministic output.

## Public API

- One declared formula extracted from a member `Cargo.toml`.
- `pub fn discover_formulas` — Walk `workspace_root` (depth ≤ 3) for member `Cargo.toml` files and

## Related

- parent: `kei-arch-derive/Cargo.toml`
- imports: anyhow, kei_registry, serde, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
