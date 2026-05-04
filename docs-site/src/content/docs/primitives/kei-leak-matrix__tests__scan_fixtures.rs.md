---
title: scan_fixtures.rs
path: kei-leak-matrix/tests/scan_fixtures.rs
dna_hash: sha256:b71cff59c43f612f
language: rust
size_loc: 167
generated: by-keidocs
---

# kei-leak-matrix/tests/scan_fixtures.rs

Scanner integration tests.

Tests use a synthetic in-test matrix written to a temp file. We do NOT
echo any real SSoT pattern. Test fixtures use clearly-test-only tokens
(TESTONLY_* prefixes) that the SSoT matrix does not list.

Where we cross-check the real SSoT matrix, we reference rules by `id`
only — never by pattern source.

## Public API

- Build a tiny test matrix on disk and return the loaded Matrix.

## Related

- parent: `kei-leak-matrix/tests`
- imports: kei_leak_matrix, std, tempfile

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
