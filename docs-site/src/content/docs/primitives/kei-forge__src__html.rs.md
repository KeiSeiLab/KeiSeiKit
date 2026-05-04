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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
