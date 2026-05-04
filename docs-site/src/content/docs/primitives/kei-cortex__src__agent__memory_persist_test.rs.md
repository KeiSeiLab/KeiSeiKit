---
title: memory_persist_test.rs
path: kei-cortex/src/agent/memory_persist_test.rs
dna_hash: sha256:1f8cb8c7436e0217
language: rust
size_loc: 97
generated: by-keidocs
---

# kei-cortex/src/agent/memory_persist_test.rs

Inline unit tests for `memory_persist.rs`.

Constructor Pattern: extracted to a sibling so the parent stays
≤200 LOC. Tests cover the pure classifier and the on-disk write
path against a fresh tempfile sqlite.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: kei_pet, tempfile

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
