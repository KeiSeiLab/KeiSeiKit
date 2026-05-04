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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
