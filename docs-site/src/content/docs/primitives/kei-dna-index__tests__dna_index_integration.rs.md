---
title: dna_index_integration.rs
path: kei-dna-index/tests/dna_index_integration.rs
dna_hash: sha256:f91912acf2e198ea
language: rust
size_loc: 405
generated: by-keidocs
---

# kei-dna-index/tests/dna_index_integration.rs

Integration tests for kei-dna-index.

Each test builds a minimal `agents` table in a tempfile sqlite DB,
then opens it read-only via the library and asserts public-API behaviour.

## Related

- parent: `kei-dna-index/tests`
- imports: kei_dna_index, rusqlite, tempfile

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
