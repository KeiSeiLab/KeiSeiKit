---
title: mod.rs
path: kei-hibernate/tests/common/mod.rs
dna_hash: sha256:5981e211a23b93c9
language: rust
size_loc: 69
generated: by-keidocs
---

# kei-hibernate/tests/common/mod.rs

Shared fixtures + bundle-crafting helpers for kei-hibernate tests.
Constructor Pattern keeps helper code in one cube, tests split by topic.

## Public API

- Signal-only consumer so `rustc` does not flag `MANIFEST_VERSION`

## Related

- parent: `kei-hibernate/tests/common`
- imports: kei_hibernate, std, tempfile

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
