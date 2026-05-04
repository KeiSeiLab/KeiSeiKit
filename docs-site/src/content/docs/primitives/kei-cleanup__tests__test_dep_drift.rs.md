---
title: test_dep_drift.rs
path: kei-cleanup/tests/test_dep_drift.rs
dna_hash: sha256:f0a0b334a78ceff9
language: rust
size_loc: 79
generated: by-keidocs
---

# kei-cleanup/tests/test_dep_drift.rs

Integration test for the dep_drift scanner.

Builds a fixture workspace whose member crate pins `serde = "1.0.180"`
while `[workspace.dependencies]` declares `serde = "1.0.190"`.

## Related

- parent: `kei-cleanup/tests`
- imports: kei_cleanup, std, tempfile

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
