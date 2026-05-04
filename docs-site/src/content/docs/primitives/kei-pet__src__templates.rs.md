---
title: templates.rs
path: kei-pet/src/templates.rs
dna_hash: sha256:73f9048bb5d0ba3f
language: rust
size_loc: 55
generated: by-keidocs
---

# kei-pet/src/templates.rs

Preset pet persona templates.

Each template is a bundled, schema-valid TOML seed parsed at runtime
via `crate::parse`. The set intentionally covers five distinct
personas so `/pet-setup` can offer one-click starting points.

## Public API

- The five preset persona archetypes shipped with kei-pet.
- `pub fn load_template` — Load a preset template and return the fully-validated manifest.
- `pub fn list_templates` — Stable-ordered list of templates with short human descriptions.

## Related

- parent: `kei-pet/Cargo.toml`
- imports: crate

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
