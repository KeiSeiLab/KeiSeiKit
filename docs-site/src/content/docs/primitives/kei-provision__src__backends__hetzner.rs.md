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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
