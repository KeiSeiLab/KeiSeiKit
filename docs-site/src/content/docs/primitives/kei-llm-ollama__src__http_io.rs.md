---
title: http_io.rs
path: kei-llm-ollama/src/http_io.rs
dna_hash: sha256:0e112192ddba5aef
language: rust
size_loc: 35
generated: by-keidocs
---

# kei-llm-ollama/src/http_io.rs

Low-level HTTP helpers shared by `Client`.

Keeps `client.rs` under the Constructor-Pattern 200-LOC limit by hosting
the per-call decode + status-check primitives here.

## Public API

- Read body, decode JSON, otherwise translate to ApiError.
- `pub fn check_status` — Map non-2xx status to `ApiError`. 404 → `ModelNotFound` (caller path is hint).

## Related

- parent: `kei-llm-ollama/Cargo.toml`
- imports: crate, serde

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
