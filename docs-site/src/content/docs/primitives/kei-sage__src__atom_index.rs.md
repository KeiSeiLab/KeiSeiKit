---
title: atom_index.rs
path: kei-sage/src/atom_index.rs
dna_hash: sha256:67ac488352a30f71
language: rust
size_loc: 63
generated: by-keidocs
---

# kei-sage/src/atom_index.rs

Persist discovered atoms into the kei-sage Store as Units + typed edges.

Unit-type = `"atom"`; `vault_path` = atom full_id (e.g. `kei-task::create`).
Edge-type = `"atom_related"` for wikilinks between atoms. Idempotent:
re-ingesting the same corpus replaces existing rows by vault_path.

## Related

- parent: `kei-sage/Cargo.toml`
- imports: anyhow, crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
