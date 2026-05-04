---
title: manifest.rs
path: kei-hibernate/src/manifest.rs
dna_hash: sha256:a8b1e2b1da3cba32
language: rust
size_loc: 54
generated: by-keidocs
---

# kei-hibernate/src/manifest.rs

hibernate-manifest.toml schema.

One manifest entry per file in the bundle. `machine_id` captures
the source host so operators can detect cross-machine restores.
Version gate blocks imports from future/incompatible primitives.

## Public API

- Path inside the bundle (forward-slash normalised, relative
- Hex-encoded SHA-256 digest of file bytes at export time.
- File size in bytes (pre-compression). Sanity check; not
- `pub fn lookup` — Locate an entry by bundle-relative path.

## Related

- parent: `kei-hibernate/Cargo.toml`
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
