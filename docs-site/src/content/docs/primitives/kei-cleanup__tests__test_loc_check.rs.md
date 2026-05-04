---
title: test_loc_check.rs
path: kei-cleanup/tests/test_loc_check.rs
dna_hash: sha256:dacaa457dcfb46b7
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-cleanup/tests/test_loc_check.rs

Integration test for the loc_check scanner.

Builds a temporary workspace with one oversized file and verifies
the scanner emits a `LocFile` finding plus an over-30-LOC `LocFunction`
finding for the embedded function.

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
