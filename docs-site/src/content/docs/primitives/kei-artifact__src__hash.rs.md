---
title: hash.rs
path: kei-artifact/src/hash.rs
dna_hash: sha256:e3e84df94e91f4b7
language: rust
size_loc: 54
generated: by-keidocs
---

# kei-artifact/src/hash.rs

sha256-based artifact id.

Id = sha256(schema_name || 0x00 || content_bytes). Including the schema
name prevents trivial collisions across different content types with the
same payload bytes. Hex-encoded 64-char string.

## Public API

- `pub fn artifact_id` — Deterministic artifact id from schema name + content bytes.

## Related

- parent: `kei-artifact/Cargo.toml`
- imports: sha2

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
