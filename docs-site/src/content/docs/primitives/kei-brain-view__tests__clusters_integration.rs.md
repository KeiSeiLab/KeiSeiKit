---
title: clusters_integration.rs
path: kei-brain-view/tests/clusters_integration.rs
dna_hash: sha256:cd951895ae43fea5
language: rust
size_loc: 123
generated: by-keidocs
---

# kei-brain-view/tests/clusters_integration.rs

Integration tests for `render_clusters`.

Each test seeds a minimal kei-ledger-compatible `agents` table with
canonical DNAs (`<role>::<caps>::<sha8-scope>::<sha8-body>-<hex8-nonce>`)
and asserts on the rendered ASCII block.

## Related

- parent: `kei-brain-view/tests`
- imports: kei_brain_view, kei_dna_index, rusqlite, tempfile

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
