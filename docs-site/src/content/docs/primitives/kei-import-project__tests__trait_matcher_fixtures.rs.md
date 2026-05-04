---
title: trait_matcher_fixtures.rs
path: kei-import-project/tests/trait_matcher_fixtures.rs
dna_hash: sha256:d74f3c8627ef9457
language: rust
size_loc: 114
generated: by-keidocs
---

# kei-import-project/tests/trait_matcher_fixtures.rs

A2.1 integration tests: validate trait-pattern matching against real sibling crates.

Each positive fixture asserts that match_module() detects the expected
TraitKind with confidence >= 0.5 when given a real crate's source files.
Negative fixtures assert that utility crates produce no confident matches.

## Related

- parent: `kei-import-project/tests`
- imports: kei_import_project, std

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
