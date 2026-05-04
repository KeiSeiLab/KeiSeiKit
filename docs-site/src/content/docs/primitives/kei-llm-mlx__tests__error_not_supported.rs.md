---
title: error_not_supported.rs
path: kei-llm-mlx/tests/error_not_supported.rs
dna_hash: sha256:18804bdaa9c86c20
language: rust
size_loc: 46
generated: by-keidocs
---

# kei-llm-mlx/tests/error_not_supported.rs

On non-target builds, `generate()` MUST return `Error::NotSupported`
before any subprocess attempt — observable via the cfg! gate.

On target builds (macos+aarch64) the test exercises the parallel
shape: it asserts that `is_supported()` agrees with the cfg! gate
AND that `build_spec` over a remote host still refuses with
`Error::SecurityRefused`. Together this keeps the test useful on
every host while pinning the not-supported semantics on every
non-target build.

## Related

- parent: `kei-llm-mlx/tests`
- imports: kei_llm_mlx

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
