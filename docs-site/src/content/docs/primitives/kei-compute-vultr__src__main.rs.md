---
title: main.rs
path: kei-compute-vultr/src/main.rs
dna_hash: sha256:be7fa9b9ee3f48a7
language: rust
size_loc: 146
generated: by-keidocs
---

# kei-compute-vultr/src/main.rs

kei-compute-vultr — CLI front-end for the Vultr Cloud v2 ComputeProvider.

Subcommands mirror kei-compute-hetzner:
dna          — print the provider's DNA
cloud-init   — render YAML cloud-init from CLI args
provision    — call POST /instances (real API, requires VULTR_API_KEY)
status       — call GET /instances/<id>

## Public API

- Print the constructor DNA.
- Render a cloud-init YAML to stdout.
- Provision a Vultr instance (live API call).
- Get current status of a previously-provisioned instance.

## Related

- parent: `kei-compute-vultr/Cargo.toml`
- imports: clap, kei_compute_vultr, kei_runtime_core

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
