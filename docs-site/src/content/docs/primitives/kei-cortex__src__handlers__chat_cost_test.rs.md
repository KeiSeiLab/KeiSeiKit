---
title: chat_cost_test.rs
path: kei-cortex/src/handlers/chat_cost_test.rs
dna_hash: sha256:d7d85ed291dbb8a1
language: rust
size_loc: 151
generated: by-keidocs
---

# kei-cortex/src/handlers/chat_cost_test.rs

Inline unit tests for `chat_cost.rs` — the pure-helper subset.

Constructor Pattern: ledger-roundtrip tests live in
`chat_cost_test_ledger.rs` so each test file stays under the 200-LOC
ceiling after Wave 44c added micro-cents + accumulation + poison
recovery test coverage.

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
