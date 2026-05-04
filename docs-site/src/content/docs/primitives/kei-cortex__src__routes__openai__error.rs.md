---
title: error.rs
path: kei-cortex/src/routes/openai/error.rs
dna_hash: sha256:c63cd648edc336d3
language: rust
size_loc: 92
generated: by-keidocs
---

# kei-cortex/src/routes/openai/error.rs

OpenAI-style error envelope `{ "error": { message, type, code } }`.

Local to the `/v1/*` surface so we can match the OpenAI wire format
exactly without leaking it into the existing kei-cortex `AppError`
(which uses the `{ "error": { code, message } }` shape).

## Public API

- Local error type for `/v1/*` handlers. Each variant becomes an

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum

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
