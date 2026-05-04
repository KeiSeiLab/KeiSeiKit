---
title: smoke.rs
path: kei-graph-stream/tests/smoke.rs
dna_hash: sha256:092fe7889dd15ff9
language: rust
size_loc: 143
generated: by-keidocs
---

# kei-graph-stream/tests/smoke.rs

## Public API

- Integration smoke test: spins up a real kei-graph-stream server on a random port,
- Write the test token to `<home>/.keisei/cortex.token` and set HOME.
- Build an authenticated WS request.

## Related

- parent: `kei-graph-stream/tests`
- imports: axum, futures, serde_json, std, tempfile, tokio, tokio_tungstenite

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
