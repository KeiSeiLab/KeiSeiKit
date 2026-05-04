---
title: models.rs
path: kei-cortex/src/routes/openai/models.rs
dna_hash: sha256:0e11d45e7ebd910d
language: rust
size_loc: 75
generated: by-keidocs
---

# kei-cortex/src/routes/openai/models.rs

GET /v1/models — list the (single) model the daemon advertises.

We expose ONE model id, configurable via `KEI_MODEL_NAME` (default
`kei-cortex`). Multi-model exposure would imply provider routing at
the OpenAI surface, which is a Phase-2 concern — for now any frontend
sees one entry and uses it as the `model` field on chat-completions.

## Public API

- Env var consulted at request time so a daemon restart isn't needed
- Default model id when the env var is unset.
- Handler for `GET /v1/models`.
- `pub fn current_model_name` — Read `KEI_MODEL_NAME`, falling back to the default. Empty / whitespace
- Unix-time seconds, with a safe fallback to 0 instead of `unwrap()`.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, std

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
