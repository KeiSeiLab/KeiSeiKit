---
title: error_nonzero_exit.rs
path: kei-llm-mlx/tests/error_nonzero_exit.rs
dna_hash: sha256:d477de2f2dee33d4
language: rust
size_loc: 37
generated: by-keidocs
---

# kei-llm-mlx/tests/error_nonzero_exit.rs

`generate()` translates a non-zero subprocess exit into
`Error::NonZeroExit { code, stderr }` with the stderr captured.

On non-target builds, `generate()` short-circuits with NotSupported
before reaching the runner — so the assertion is wrapped in a
cfg-gate. On target builds we exercise the full path with a mock
that returns exit=2 + a stderr message.

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
