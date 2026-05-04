---
title: quality_tests_green.rs
path: kei-agent-runtime/src/verifies/quality_tests_green.rs
dna_hash: sha256:da03aa77cc9549fe
language: rust
size_loc: 80
generated: by-keidocs
---

# kei-agent-runtime/src/verifies/quality_tests_green.rs

`quality::tests-green` — runs `cargo test -p <crate>` for each crate in
`task.verification.cargo-test-crates`; parses `test result: ok. N passed`
lines; asserts total count ≥ `test_count_min` when set.

As of v0.18 convergence wave: `CommandVerify` wrapper with a custom
per-crate runner (default exit-check shape doesn't fit the loop).

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: crate, once_cell, regex, std

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
