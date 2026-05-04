---
title: atoms_discover_smoke.rs
path: kei-sage/tests/atoms_discover_smoke.rs
dna_hash: sha256:b91a752ce5ecabb8
language: rust
size_loc: 134
generated: by-keidocs
---

# kei-sage/tests/atoms_discover_smoke.rs

Integration smoke test for atom discovery + wikilink resolution.

Creates a temp root with 2 fake crates, each with `atoms/<verb>.md`,
asserts `discover_atoms` returns 2 records and frontmatter is parsed.

## Related

- parent: `kei-sage/tests`
- imports: kei_sage, std, tempfile

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
