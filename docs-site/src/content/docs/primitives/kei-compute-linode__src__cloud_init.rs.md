---
title: cloud_init.rs
path: kei-compute-linode/src/cloud_init.rs
dna_hash: sha256:ab467697eb268143
language: rust
size_loc: 83
generated: by-keidocs
---

# kei-compute-linode/src/cloud_init.rs

Cloud-init renderer for Linode. Linode's `metadata.user_data` field
requires the user-data blob to be **base64-encoded**, so we expose
both the raw render (`render`) and the API-ready encoded form
(`render_base64`).

## Public API

- Minimal cloud-init template: hostname, ssh authorized_keys, runcmd.
- `pub fn render` — Render the cloud-init YAML body. Deterministic — no I/O, no time.
- `pub fn render_base64` — Render and base64-encode for `metadata.user_data`.

## Related

- parent: `kei-compute-linode/Cargo.toml`
- imports: base64

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
