---
title: firmware.rs
path: frustration-matrix/tests/firmware.rs
dna_hash: sha256:d7557947fe73fdb9
language: rust
size_loc: 124
generated: by-keidocs
---

# frustration-matrix/tests/firmware.rs

Firmware tests — cover training, save/load, multilingual alphabet,
unigram fallback, and size budget (≤50 KB at depth 4 on 1 MB corpus).

Like `tests/integration.rs`, we link source modules via `#[path]` so
the binary crate doesn't need to export a library surface.

## Related

- parent: `frustration-matrix/tests`
- imports: firmware, std, tempfile

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
