---
title: test_unused_deps.rs
path: kei-cleanup/tests/test_unused_deps.rs
dna_hash: sha256:dff14556d10775e1
language: rust
size_loc: 50
generated: by-keidocs
---

# kei-cleanup/tests/test_unused_deps.rs

Integration test for the unused_deps scanner.

Builds a tmp crate with `tracing` declared in Cargo.toml but no
`use tracing` or `tracing::` reference in src/.

## Related

- parent: `kei-cleanup/tests`
- imports: kei_cleanup, serde, std, tempfile

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
