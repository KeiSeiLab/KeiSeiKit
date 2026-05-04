---
title: integration.rs
path: kei-memory/tests/integration.rs
dna_hash: sha256:a175348a91168772
language: rust
size_loc: 172
generated: by-keidocs
---

# kei-memory/tests/integration.rs

Integration tests for kei-memory.

Constructor Pattern: each test = one scenario, one assertion target.
Uses tempfile for per-test isolated sqlite file. Imports the
library crate directly (kei-memory now exposes [lib] + [bin]).

## Related

- parent: `kei-memory/tests`
- imports: kei_memory, rusqlite, std, tempfile

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
