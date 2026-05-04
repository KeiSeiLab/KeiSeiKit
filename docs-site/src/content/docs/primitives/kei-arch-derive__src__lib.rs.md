---
title: lib.rs
path: kei-arch-derive/src/lib.rs
dna_hash: sha256:78031cd8551c361f
language: rust
size_loc: 23
generated: by-keidocs
---

# kei-arch-derive/src/lib.rs

kei-arch-derive — bridge between kei-registry formulas and the
canonical `arch/PLAN.toml` evidence file.

Phase 2 PR-3 of `arch/MATH-DNA-DESIGN.md`. Reads the registry SQLite,
walks the workspace for `[package.metadata.keisei.formula]` declarations
in member `Cargo.toml`s, and projects the formula 4-tuple onto the
seven `kei_arch_map::schema::Evidence` kinds already shipped in PR-1.

Constructor Pattern: each module is one cube with one responsibility.
`project` owns the predicate→evidence mapping table; `walker` owns
Cargo.toml discovery; `emit` owns deterministic TOML serialisation;
`coverage` owns the 2-axis (presence + agreement) metric.

## Related

- parent: `kei-arch-derive/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
