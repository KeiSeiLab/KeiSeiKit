---
title: lib.rs
path: kei-compute-linode/src/lib.rs
dna_hash: sha256:dd573352c605d21a
language: rust
size_loc: 25
generated: by-keidocs
---

# kei-compute-linode/src/lib.rs

kei-compute-linode — ComputeProvider impl for Linode (Akamai Cloud) v4 API.

Sibling of kei-compute-hetzner. Wave 2 atomar provider crate.

- `api`        — thin HTTP client over `https://api.linode.com/v4`
- `cloud_init` — render + base64-encode user-data for `metadata.user_data`
- `provider`   — `LinodeCompute: ComputeProvider` (DNA + tier policy + status map)
- `error`      — local error type, mapped into `kei_runtime_core::Error`

Auth: `Authorization: Bearer $LINODE_TOKEN` (env-only, RULE 0.8).

## Related

- parent: `kei-compute-linode/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
