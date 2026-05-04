---
title: lib.rs
path: kei-compute-baremetal/src/lib.rs
dna_hash: sha256:89a2da36d5b9b98e
language: rust
size_loc: 18
generated: by-keidocs
---

# kei-compute-baremetal/src/lib.rs

kei-compute-baremetal — ComputeProvider impl for user-owned hardware.

Registers an existing SSH-reachable box (VPS, dedicated server, lab box)
as a managed VM. `create()` runs the cloud-init shell over SSH; `destroy()`
deregisters but never powers off user hardware. `resize()` / `start()` /
`stop()` return `NotImplemented` — manual user action only.

Auth: SSH key path is passed at construction (RULE 0.8 — never hardcoded).

## Related

- parent: `kei-compute-baremetal/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
