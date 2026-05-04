---
title: error.rs
path: keisei/src/error.rs
dna_hash: sha256:f65a3e243b04b9e1
language: rust
size_loc: 97
generated: by-keidocs
---

# keisei/src/error.rs

Error type for the `keisei` CLI.

Constructor Pattern: single responsibility — own all failure modes of the
attach / status / mount / detach flow as one thiserror enum. Every other
module returns `Result<T, Error>` using the `#[from]` conversions here.

## Related

- parent: `keisei/Cargo.toml`
- imports: std

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
