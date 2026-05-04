---
title: wikilink.rs
path: kei-atom-discovery/src/wikilink.rs
dna_hash: sha256:c7b4f031461938f9
language: rust
size_loc: 61
generated: by-keidocs
---

# kei-atom-discovery/src/wikilink.rs

Wikilink parsing and classification.

Covers the strict `[[target]]` matcher used by `kei-sage` and
`kei-runtime` to link atom docs to each other and to rule files.

## Public API

- `pub fn parse_wikilink` — Parse a single wikilink `[[target]]`. Returns `None` if not a wikilink,
- `pub fn is_atom_target` — Heuristic atom-target filter: `<crate>::<verb>` looks like an atom,
- Classified wikilink target — atom, rule reference, or other (notes etc.).
- `pub fn classify_wikilink` — Classify a wikilink inner body. `inner` is the already-unwrapped target
- Normalise the tail after `rules/` or `rule ` into a short slug.

## Related

- parent: `kei-atom-discovery/Cargo.toml`

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
