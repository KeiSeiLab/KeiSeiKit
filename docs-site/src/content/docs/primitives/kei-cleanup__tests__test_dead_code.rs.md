---
title: test_dead_code.rs
path: kei-cleanup/tests/test_dead_code.rs
dna_hash: sha256:75bf8cd35fa1691d
language: rust
size_loc: 47
generated: by-keidocs
---

# kei-cleanup/tests/test_dead_code.rs

Integration test for the dead_code scanner shell-out.

Skips itself silently when neither `cargo-machete` nor
`cargo-udeps` is available on PATH (per CLEANUP-RUNTIME-SPEC.md
§8.3 graceful fallback). When `cargo-machete` IS available, runs
it on a fixture and asserts the scanner returns Ok or
ToolNotFound — never panics.

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
