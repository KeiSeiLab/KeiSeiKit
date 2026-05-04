---
title: schema.rs
path: kei-pet/src/schema.rs
dna_hash: sha256:9069e6e8e3cb138b
language: rust
size_loc: 317
generated: by-keidocs
---

# kei-pet/src/schema.rs

Schema types for pet.toml.

Enums use `#[serde(rename_all = "kebab-case")]` to match the TOML wire
format (e.g. `"mirror-user"`). Optional fields use `Option<T>` and are
omitted on serialize when `None`. Arrays default to `Vec::new()`.

## Public API

- Schema version. Must be `1` for this crate.

## Related

- parent: `kei-pet/Cargo.toml`
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
