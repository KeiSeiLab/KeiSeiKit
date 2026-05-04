---
title: anthropic_stream_shape.rs
path: kei-router/tests/anthropic_stream_shape.rs
dna_hash: sha256:5ad9bf1a1f8a25af
language: rust
size_loc: 88
generated: by-keidocs
---

# kei-router/tests/anthropic_stream_shape.rs

Anthropic provider streaming wire-shape test.

Stands up a wiremock fake of /v1/messages, replies with a hand-crafted SSE
body, asserts the StreamEvent sequence the parser produces.

## Related

- parent: `kei-router/tests`
- imports: futures, kei_router, wiremock

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
