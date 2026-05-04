---
title: generate_nonstream.rs
path: kei-llm-mlx/tests/generate_nonstream.rs
dna_hash: sha256:80d462324b30af3e
language: rust
size_loc: 41
generated: by-keidocs
---

# kei-llm-mlx/tests/generate_nonstream.rs

Generate — non-streaming stdout parsing.

mlx_lm prints the generated text, then `==========` separator, then a
footer with token / tokens-per-sec stats. We pin the parser against
a representative fixture so version drift in mlx_lm output style
is caught here, not in production.

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
