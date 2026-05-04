---
title: discover.rs
path: kei-runtime/src/discover.rs
dna_hash: sha256:9f3f334348f62523
language: rust
size_loc: 20
generated: by-keidocs
---

# kei-runtime/src/discover.rs

Atom discovery — thin façade over `kei-atom-discovery`.

Re-exports `AtomMeta` and `AtomKind` from the shared crate so all runtime
modules share exactly one frontmatter-parser implementation.

## Public API

- `pub fn walk_atoms` — Walk `<root>/*/atoms/*.md`. Delegates to `kei-atom-discovery::discover_atoms`.
- `pub fn extract_frontmatter` — Backwards-compatible split — returns the frontmatter YAML body (no body

## Related

- parent: `kei-runtime/Cargo.toml`
- imports: kei_atom_discovery, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
