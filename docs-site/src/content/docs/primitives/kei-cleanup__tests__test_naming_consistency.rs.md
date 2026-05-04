---
title: test_naming_consistency.rs
path: kei-cleanup/tests/test_naming_consistency.rs
dna_hash: sha256:d0b88404a256c951
language: rust
size_loc: 79
generated: by-keidocs
---

# kei-cleanup/tests/test_naming_consistency.rs

Integration test for the naming_consistency scanner.

Tmp crate contains both `D_INIT` and `DEFAULT_D` constants. cleanup
config declares them as a synonym pair → drift should be flagged.

## Related

- parent: `kei-cleanup/tests`
- imports: kei_cleanup, std, tempfile

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
