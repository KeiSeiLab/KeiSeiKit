---
title: normalizer_unified.rs
path: kei-decompose/tests/normalizer_unified.rs
dna_hash: sha256:191fabc785320893
language: rust
size_loc: 56
generated: by-keidocs
---

# kei-decompose/tests/normalizer_unified.rs

Cross-parser normalization — every parser yields the same Action shape.

This catches drift: if a new adapter starts emitting half-filled Actions
or skips the source_format tag, downstream emit/dispatch breaks silently.

## Related

- parent: `kei-decompose/tests`
- imports: kei_decompose, std

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
