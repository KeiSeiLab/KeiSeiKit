---
title: error.rs
path: kei-net-openvpn/src/error.rs
dna_hash: sha256:d8610526c2f4bca3
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-net-openvpn/src/error.rs

Crate-local error. Bridges into `kei_runtime_core::Error` so
`NetworkMode` impl methods can `?` through this and surface a
substrate-level error to callers without leaking systemctl /
management-socket transport details.

## Public API

- Bridge into substrate-level error. OpenVPN-specific failures are

## Related

- parent: `kei-net-openvpn/Cargo.toml`
- imports: thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
