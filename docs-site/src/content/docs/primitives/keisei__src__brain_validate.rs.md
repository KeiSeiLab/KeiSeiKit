---
title: brain_validate.rs
path: keisei/src/brain_validate.rs
dna_hash: sha256:9a7a28d22032d457
language: rust
size_loc: 119
generated: by-keidocs
---

# keisei/src/brain_validate.rs

Validation primitives for `Brain::load`.

Constructor Pattern: single responsibility — own the five mechanical
checks (symlink reject / root canonicalize / manifest read / name
regex / in-root path guard). `brain.rs` composes them into the load
pipeline. No cross-module state; every fn is pure w.r.t. filesystem.

## Public API

- `pub const MAX_MANIFEST_BYTES` — L12 (v0.19.2 audit): cap manifest.toml at 64 KiB. A well-formed brain
- `pub fn check_relative_in_root` — Syntactic check before touching disk: absolute path or `..` component
- `pub fn canonicalize_in_root` — Resolve + canonicalize a manifest-declared relative path and assert it

## Related

- parent: `keisei/Cargo.toml`
- imports: crate, regex, std

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
