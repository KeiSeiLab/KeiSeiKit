---
title: hetzner.rs
path: kei-provision/src/backends/hetzner.rs
dna_hash: sha256:8c73ac712a5cfc66
language: rust
size_loc: 143
generated: by-keidocs
---

# kei-provision/src/backends/hetzner.rs

Hetzner Cloud adapter. Shells out to `hcloud server …`.

JSON shape (hcloud v1.44):
describe → `{ "id": u64, "name": str, "status": str,
"public_net": { "ipv4": { "ip": str } }, ... }`
list     → `[ { same shape } ]`
create   → `{ "server": { same shape as describe } }`

## Related

- parent: `kei-provision/Cargo.toml`
- imports: anyhow, crate, serde_json

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
