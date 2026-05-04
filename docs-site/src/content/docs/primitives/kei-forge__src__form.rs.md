---
title: form.rs
path: kei-forge/src/form.rs
dna_hash: sha256:39773d0c07c815af
language: rust
size_loc: 231
generated: by-keidocs
---

# kei-forge/src/form.rs

Form request deserialization + validation.

Accepts either `application/x-www-form-urlencoded` (HTML `<form>`) or
`application/json` (future API clients). Validation enforces the
locked substrate schema — verb naming (kebab-case) and atom kind
enumeration (command | query | stream | transform).

## Public API

- Incoming POST /forge body.
- `pub fn validate` — Validation outcome. `Ok(())` if the request matches schema constraints.
- Description whitelist — ASCII printable only.
- Matches regex `^[a-z][a-z0-9]*(-[a-z0-9]+)*$` without pulling in `regex`.

## Related

- parent: `kei-forge/Cargo.toml`
- imports: serde

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
