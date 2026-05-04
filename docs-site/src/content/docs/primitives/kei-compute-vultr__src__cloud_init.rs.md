---
title: cloud_init.rs
path: kei-compute-vultr/src/cloud_init.rs
dna_hash: sha256:d43f04202180d3c6
language: rust
size_loc: 141
generated: by-keidocs
---

# kei-compute-vultr/src/cloud_init.rs

Cloud-init renderer for Vultr instances. Same field surface as the
Hetzner sibling. Vultr's API requires the user_data payload be
base64-encoded — `render_base64()` returns the wire-ready form.

## Public API

- Inputs to render a YAML cloud-init document for a managed VM.
- `pub fn render` — Render plain YAML. Whitespace-stable, deterministic for a given
- `pub fn render_base64` — Vultr v2 demands base64 in the `user_data` field. STANDARD alphabet
- `pub fn encode_base64` — Tiny self-contained STANDARD base64 encoder. Avoids pulling the

## Related

- parent: `kei-compute-vultr/Cargo.toml`
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
