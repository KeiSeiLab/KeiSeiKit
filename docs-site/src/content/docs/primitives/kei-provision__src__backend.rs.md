---
title: backend.rs
path: kei-provision/src/backend.rs
dna_hash: sha256:aab14fde2b468b6b
language: rust
size_loc: 58
generated: by-keidocs
---

# kei-provision/src/backend.rs

Backend trait + shared data types for the unified provisioner.

A `Backend` shells out to an external CLI (hcloud / vultr-cli / future
aws / doctl / linode-cli). All IO is through the `Backend` methods;
`main.rs` never touches `std::process::Command` directly.

## Public API

- Opts passed to `Backend::create`. Fields are `Option` because every
- e.g. `cx22` (hetzner), `vc2-1c-1gb` (vultr).
- e.g. `fsn1` (hetzner), `ams` (vultr).
- e.g. `debian-12` (hetzner), `2136` (vultr os-id).
- SSH key id/name (backend-specific).
- Firewall name (hetzner) / group id (vultr).
- Cloud-init user-data file path.
- Normalized server info across all backends.
- Raw backend JSON for details the normalized fields drop
- `pub trait Backend` — Implemented by each cloud provider adapter.
- Short identifier — `"hetzner"`, `"vultr"`.
- Create a server with `name` or return existing (idempotent).
- `Ok(None)` if absent; never fails on absence.
- Idempotent: absent server = Ok(()). `force` skips confirm prompt.
- All servers owned by the current token.

## Related

- parent: `kei-provision/Cargo.toml`
- imports: anyhow, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
