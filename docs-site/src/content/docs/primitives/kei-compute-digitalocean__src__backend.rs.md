---
title: backend.rs
path: kei-compute-digitalocean/src/backend.rs
dna_hash: sha256:0cc83005622b6e24
language: rust
size_loc: 172
generated: by-keidocs
---

# kei-compute-digitalocean/src/backend.rs

[`DigitalOceanBackend`] — DNA-bearing [`ComputeProvider`] impl.

Maps DO droplet operations onto the runtime-core trait surface. The
`external_id` on a [`VmHandle`] is the droplet's numeric `id` formatted
as a string.

## Public API

- `pub const DEFAULT_IMAGE` — Default DO image when caller does not encode it in `tier`.
- DigitalOcean backend. `parent` is the operator/owner DNA (optional).
- `pub fn new` — Build a backend with a fresh DNA serial. `image` defaults to
- `pub fn map_status` — Map raw DO status string → [`VmStatus`].

## Related

- parent: `kei-compute-digitalocean/Cargo.toml`
- imports: async_trait, crate, kei_runtime_core, std

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
