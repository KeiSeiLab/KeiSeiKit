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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
