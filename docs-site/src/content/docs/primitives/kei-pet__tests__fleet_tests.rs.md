---
title: fleet_tests.rs
path: kei-pet/tests/fleet_tests.rs
dna_hash: sha256:cc873b72b5c16c9f
language: rust
size_loc: 83
generated: by-keidocs
---

# kei-pet/tests/fleet_tests.rs

Hermetic tests for the multi-pet fleet module.

Every test uses a fresh `tempfile::TempDir` as the fleet_root, so no
test touches real user state and no test depends on another's side
effects.

## Related

- parent: `kei-pet/tests`
- imports: kei_pet, std

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
