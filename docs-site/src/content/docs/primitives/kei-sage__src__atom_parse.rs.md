---
title: atom_parse.rs
path: kei-sage/src/atom_parse.rs
dna_hash: sha256:b28ca36bd2056620
language: rust
size_loc: 84
generated: by-keidocs
---

# kei-sage/src/atom_parse.rs

Sage-local aliases over `kei-atom-discovery` helpers.

Historical sage API: `split_frontmatter`, `parse_wikilink`, `is_atom_target`,
`split_atom_id`. All now delegate to the shared crate; kept here so sage
internals compile without touch.

## Public API

- `pub fn split_frontmatter` — Split a `.md` file into (frontmatter_yaml, body). Delegates to the shared
- `pub fn split_atom_id` — Split `<crate>::<verb>` atom id into components.

## Related

- parent: `kei-sage/Cargo.toml`
- imports: anyhow, kei_atom_discovery

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
