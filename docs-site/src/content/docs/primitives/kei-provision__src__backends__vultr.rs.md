---
title: vultr.rs
path: kei-provision/src/backends/vultr.rs
dna_hash: sha256:dd6d46e3b2b7d155
language: rust
size_loc: 189
generated: by-keidocs
---

# kei-provision/src/backends/vultr.rs

Vultr adapter. Shells out to `vultr-cli instance …` (v3 CLI).

JSON shape (vultr-cli v3):
instance list → `{ "instances": [ { "id": str, "label": str,
"main_ip": str, "status": str, "region": str,
"plan": str, "power_status": str, ... } ] }`
instance get <id> → `{ "instance": {...} }` (id required, not label)
instance create → `{ "instance": {...} }`
os list → `{ "os": [ { "id": int, "name": str, ... } ] }`

## Related

- parent: `kei-provision/Cargo.toml`
- imports: anyhow, crate, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
