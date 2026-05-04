---
title: role.rs
path: kei-registry/src/scanners/role.rs
dna_hash: sha256:0f24a752f52601b1
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-registry/src/scanners/role.rs

Role scanner — walks `<kit-root>/_roles/*.toml`.

Constructor Pattern: this cube knows the flat `_roles/` directory
convention. Body = raw TOML; name = filename stem; maps to
BlockType::Atom; caps = empty.

## Public API

- `pub struct RoleScanner` — `<kit-root>/_roles/<name>.toml` adapter.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, std

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
