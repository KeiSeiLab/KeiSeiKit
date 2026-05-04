---
title: chat_memory_nudge_test.rs
path: kei-cortex/src/handlers/chat_memory_nudge_test.rs
dna_hash: sha256:8b6176649e05a9e5
language: rust
size_loc: 112
generated: by-keidocs
---

# kei-cortex/src/handlers/chat_memory_nudge_test.rs

Inline unit tests for `chat_memory_nudge.rs`.

Constructor Pattern: extracted to a sibling so the parent stays
≤200 LOC. Tests cover the context-builder and verify the wiring
ends up with both `invoker` and `persist` populated (regression
against the prior dead-code state).

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

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
