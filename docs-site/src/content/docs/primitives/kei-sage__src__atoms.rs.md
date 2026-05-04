---
title: atoms.rs
path: kei-sage/src/atoms.rs
dna_hash: sha256:06e1f0923e0e70df
language: rust
size_loc: 40
generated: by-keidocs
---

# kei-sage/src/atoms.rs

Substrate-atom discovery — thin façade over `kei-atom-discovery`.

Historical `AtomRecord` is preserved as a type alias for `AtomMeta` so
that downstream sage modules (`atom_index`, `atom_cli`) keep compiling.

## Public API

- `pub type AtomRecord` — Legacy alias: sage used to call this `AtomRecord`. New code should use
- `pub fn discover_atoms` — Walk `<root>/*/atoms/*.md` and return parsed atom metadata.
- `pub fn resolve_wikilinks` — Extract `(source_atom_id, target)` edges from `related:` wikilinks.

## Related

- parent: `kei-sage/Cargo.toml`
- imports: anyhow, crate, kei_atom_discovery, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
