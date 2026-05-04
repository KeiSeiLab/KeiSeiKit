---
title: provider.rs
path: kei-compute-baremetal/src/provider.rs
dna_hash: sha256:9e60301f4bc24300
language: rust
size_loc: 300
generated: by-keidocs
---

# kei-compute-baremetal/src/provider.rs

[`BaremetalCompute`] — `ComputeProvider` for user-owned hardware.

Differences vs cloud providers:
* `create()` does not provision — it registers an SSH connection and
runs the cloud-init shell remotely.
* `destroy()` deregisters; user hardware is never powered off.
* `resize()`/`start()`/`stop()` return `NotImplemented`.
* `cost_per_hour_microcents()` is always 0 (user-owned).

## Public API

- `pub struct BaremetalCompute` — Bare-metal provider. Stateless w.r.t. registered boxes — the SSH
- Default SSH key path applied when a `VmSpec` does not override it
- First 8 hex chars of SHA-256(input). Stable host-fingerprint for DNA caps.

## Related

- parent: `kei-compute-baremetal/Cargo.toml`
- imports: crate, kei_runtime_core, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
