---
title: html.rs
path: kei-forge/src/html.rs
dna_hash: sha256:3416b1949d9a7dc1
language: rust
size_loc: 71
generated: by-keidocs
---

# kei-forge/src/html.rs

Static HTML for the wizard form.

The form no longer POSTs `application/x-www-form-urlencoded` directly
— that body-type is SOP-safe (no CORS preflight) which allowed any
web page to CSRF POST the handler. Instead, a small inline `<script>`
serialises form values to JSON and POSTs via `fetch()` with
`Content-Type: application/json`. JSON bodies trigger CORS preflight
so the Same-Origin-Policy now engages.

## Related

- parent: `kei-forge/Cargo.toml`

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
