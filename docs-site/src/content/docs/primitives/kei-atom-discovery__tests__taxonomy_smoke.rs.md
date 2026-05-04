---
title: taxonomy_smoke.rs
path: kei-atom-discovery/tests/taxonomy_smoke.rs
dna_hash: sha256:9b571c052191040a
language: rust
size_loc: 154
generated: by-keidocs
---

# kei-atom-discovery/tests/taxonomy_smoke.rs

Taxonomy + Lineage facet parsing smoke tests.

Covers (a) full 7-facet taxonomy + lineage with multiple parents,
(b) partial taxonomy (only kingdom + mechanism) — remaining fields None,
(c) backward-compat: atom without any [taxonomy]/[lineage] still parses,
(d) lineage.parents array parses correctly (multi-parent diamond lineage).

## Related

- parent: `kei-atom-discovery/tests`
- imports: kei_atom_discovery, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
