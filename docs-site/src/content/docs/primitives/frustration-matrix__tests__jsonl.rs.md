---
title: jsonl.rs
path: frustration-matrix/tests/jsonl.rs
dna_hash: sha256:7bbd78fc722fab2b
language: rust
size_loc: 118
generated: by-keidocs
---

# frustration-matrix/tests/jsonl.rs

Integration tests for `jsonl` cube.

Constructor Pattern: one scenario per test. We mount `jsonl.rs` via
`#[path]` (same pattern as `integration.rs`) so no library crate
surface is required. Fixtures are written to `tempfile::TempDir` —
nothing persists after the test.

## Related

- parent: `frustration-matrix/tests`
- imports: jsonl, std, tempfile

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
