---
title: auto_train_threshold.rs
path: kei-frustration-loop/tests/auto_train_threshold.rs
dna_hash: sha256:bca3364a04fd0279
language: rust
size_loc: 122
generated: by-keidocs
---

# kei-frustration-loop/tests/auto_train_threshold.rs

Threshold trigger logic. Populate N-1 feedback rows → assert
`should_retrain == false`. Add one more → assert `true`. Then run the
actual `auto_train` and verify a new firmware file was written.

## Related

- parent: `kei-frustration-loop/tests`
- imports: kei_frustration_loop, std, tempfile

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
