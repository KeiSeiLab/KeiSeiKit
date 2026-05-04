---
title: integration.rs
path: kei-projects-index/tests/integration.rs
dna_hash: sha256:6768a3fd62e9e231
language: rust
size_loc: 117
generated: by-keidocs
---

# kei-projects-index/tests/integration.rs

Integration tests for kei-projects-index.

Constructor Pattern: each test = one scenario, one assertion target.
Uses `tempfile::tempdir` for per-test isolated working trees so the
tests are stable on a developer machine with a populated `~/Projects/`.

## Public API

- Init repo at `dir`, write README.md, stage + commit. Returns commit SHA.

## Related

- parent: `kei-projects-index/tests`
- imports: git2, kei_projects_index, rusqlite, std

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
