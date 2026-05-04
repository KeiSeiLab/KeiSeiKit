---
title: error.rs
path: kei-entity-store/src/error.rs
dna_hash: sha256:a3eba7639828be33
language: rust
size_loc: 72
generated: by-keidocs
---

# kei-entity-store/src/error.rs

Verb error type. Distinguishes user-input / validation failures
(map to CLI exit 2 in callers) from storage / IO failures (exit 1).

## Public API

- Typed-validation failure for a declared schema field.
- Generic not-found. `id` is rendered as text so the same variant
- `pub fn exit_code` — Exit code contract — 2 for validation / unknown verb / not found;
- `pub fn not_found_i64` — Construct a `NotFound` from an i64 id. Kept as a shim so existing
- `pub fn not_found_text` — Construct a `NotFound` from a String id (TextPk schemas).
- Map DDL-generation failures into verb errors. An unsupported

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate, thiserror

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
