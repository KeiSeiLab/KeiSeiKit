---
title: smoke.rs
path: kei-discover/tests/smoke.rs
dna_hash: sha256:cd287c3cb806b2c6
language: rust
size_loc: 160
generated: by-keidocs
---

# kei-discover/tests/smoke.rs

Integration smoke tests for the kei-discover public API.

Covers the 6 behaviours enumerated in task.toml:
1. register returns id + increments count
2. list_available excludes installed
3. mark_installed flips flag
4. search matches slug and description via FTS
5. register rejects duplicate slug
6. stats counts

All tests use `open_memory()` so they neither touch nor contend with
the on-disk default DB path.

## Related

- parent: `kei-discover/tests`
- imports: kei_discover

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
