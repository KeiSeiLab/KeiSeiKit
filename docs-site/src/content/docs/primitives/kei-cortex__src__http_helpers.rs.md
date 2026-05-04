---
title: http_helpers.rs
path: kei-cortex/src/http_helpers.rs
dna_hash: sha256:c9efaad9355f275c
language: rust
size_loc: 57
generated: by-keidocs
---

# kei-cortex/src/http_helpers.rs

Shared HTTP utilities: a process-wide `reqwest::Client` and a capped
response-body reader.

A single `reqwest::Client` is reused for all outbound calls to avoid
exhausting OS connection-table entries when the daemon is under load.
The client is initialized once via `once_cell::sync::Lazy`.

## Public API

- The process-wide HTTP client.  Shared by `anthropic`, `anthropic_invoker`,
- Error returned when a response body exceeds the caller-supplied cap.
- Read a response body up to `max_bytes`. Returns `Err(BodyTooLarge)` if
- Combined error for `read_capped`.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: futures, once_cell

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
