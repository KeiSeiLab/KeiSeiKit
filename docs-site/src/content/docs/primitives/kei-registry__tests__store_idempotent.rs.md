---
title: store_idempotent.rs
path: kei-registry/tests/store_idempotent.rs
dna_hash: sha256:a09efc96e64c0171
language: rust
size_loc: 40
generated: by-keidocs
---

# kei-registry/tests/store_idempotent.rs

Re-registering the same (path, body) returns the existing DNA. Single
row in the table; the original `created` timestamp is preserved.

## Related

- parent: `kei-registry/tests`
- imports: kei_registry, tempfile

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
