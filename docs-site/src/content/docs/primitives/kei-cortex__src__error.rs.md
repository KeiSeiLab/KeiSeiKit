---
title: error.rs
path: kei-cortex/src/error.rs
dna_hash: sha256:9d08e837641cb384
language: rust
size_loc: 87
generated: by-keidocs
---

# kei-cortex/src/error.rs

Unified error type mapped to HTTP responses with JSON body.

Handlers return `Result<T, AppError>` and axum converts the error via
`IntoResponse`. All outbound bodies share the shape
`{ "error": { "code": "...", "message": "..." } }` so the UI has a single
parser.

## Public API

- Application-level error. Variants map 1:1 to HTTP status codes.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, serde_json

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
