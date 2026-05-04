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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
