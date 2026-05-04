---
title: edit_unique_old_string.rs
path: kei-cortex/src/tool/tests/edit_unique_old_string.rs
dna_hash: sha256:a3e43cc44d5945e4
language: rust
size_loc: 90
generated: by-keidocs
---

# kei-cortex/src/tool/tests/edit_unique_old_string.rs

Validates the edit tool's unique-match guarantee:
- duplicate `old_string` without `replace_all` errors as `NotUnique`
- unique `old_string` succeeds and updates the file
- `replace_all = true` rewrites every occurrence and reports the count
- missing `old_string` errors with NotUnique
- `old_string == new_string` errors with InvalidInput

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate

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
