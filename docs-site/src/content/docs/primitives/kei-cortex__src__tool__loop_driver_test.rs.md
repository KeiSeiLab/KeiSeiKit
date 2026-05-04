---
title: loop_driver_test.rs
path: kei-cortex/src/tool/loop_driver_test.rs
dna_hash: sha256:87bcd773caf6eddf
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-cortex/src/tool/loop_driver_test.rs

Inline unit tests for `loop_driver.rs`. Extracted to a sibling so
the parent stays under the 200-LOC Constructor Pattern ceiling after
the Wave 44c CancellationToken refactor (F-HIGH-5).

## Public API

- F-HIGH-5: cancel token fires mid-turn and the loop short-circuits

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, futures, std

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
