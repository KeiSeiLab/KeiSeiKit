---
title: integration.rs
path: kei-brain-view/tests/integration.rs
dna_hash: sha256:f85a078feebb3317
language: rust
size_loc: 164
generated: by-keidocs
---

# kei-brain-view/tests/integration.rs

Integration tests for kei-brain-view.

Constructor Pattern: each test = one scenario. A helper seeds a
minimal kei-ledger-compatible schema into a tempfile sqlite, then
the library walks it read-only.

## Public API

- Seed a v4-compatible `agents` table and return (tempdir, conn).

## Related

- parent: `kei-brain-view/tests`
- imports: kei_brain_view, rusqlite, tempfile

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
