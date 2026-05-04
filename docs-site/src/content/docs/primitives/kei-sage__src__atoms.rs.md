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
