---
title: registry_write_tests.rs
path: kei-import-project/tests/registry_write_tests.rs
dna_hash: sha256:b077a3384248685a
language: rust
size_loc: 185
generated: by-keidocs
---

# kei-import-project/tests/registry_write_tests.rs

Integration tests for registry_writer: register, idempotency, supersede.

Uses tempfile for ephemeral SQLite + ephemeral repo trees. No live I/O.

## Public API

- Build a synthetic Rust mono-repo with 3 named crates.

## Related

- parent: `kei-import-project/tests`
- imports: kei_import_project, kei_registry, std, tempfile

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
