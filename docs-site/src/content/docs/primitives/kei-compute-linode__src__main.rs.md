---
title: main.rs
path: kei-compute-linode/src/main.rs
dna_hash: sha256:450516153d571ab0
language: rust
size_loc: 128
generated: by-keidocs
---

# kei-compute-linode/src/main.rs

`kei-compute-linode` CLI — thin operator surface over `LinodeCompute`.

Subcommands:
- `dna`         — print the provider DNA serial.
- `cloud-init`  — render a cloud-init blob (raw or base64).
- `provision`   — create a Linode instance from flags.
- `status`      — read instance status by id.

## Public API

- Print the provider DNA serial.
- Render a cloud-init blob.
- Create a Linode instance.
- Read instance status by id.

## Related

- parent: `kei-compute-linode/Cargo.toml`
- imports: clap, kei_compute_linode, kei_runtime_core

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
