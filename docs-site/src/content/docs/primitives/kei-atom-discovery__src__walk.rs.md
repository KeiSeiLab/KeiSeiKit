---
title: walk.rs
path: kei-atom-discovery/src/walk.rs
dna_hash: sha256:2466147dcc09e0c2
language: rust
size_loc: 91
generated: by-keidocs
---

# kei-atom-discovery/src/walk.rs

Filesystem walk for atom discovery.

`discover_atoms` enumerates `<root>/*/atoms/*.md` with `follow_links(false)`.
Malformed files emit a stderr warning and are dropped (skip-on-invalid).

## Public API

- `pub fn discover_atoms` — Walk `<root>/*/atoms/*.md`. Skip-on-invalid: malformed files emit a
- Resolve an optional schema path relative to the atom's directory.
- `pub fn split_atom_id` — Split `<crate>::<verb>` atom id into components.

## Related

- parent: `kei-atom-discovery/Cargo.toml`
- imports: crate, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
