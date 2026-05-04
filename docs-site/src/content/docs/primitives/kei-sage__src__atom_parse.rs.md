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
