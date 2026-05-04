---
title: lib.rs
path: kei-runtime-core/src/lib.rs
dna_hash: sha256:de35fc0632f0bc0c
language: rust
size_loc: 34
generated: by-keidocs
---

# kei-runtime-core/src/lib.rs

kei-runtime-core — Hosted Sleep runtime substrate.

12 capability traits + DNA + plugin registry. No backend implementations
live here; each `kei-{compute,llm,git,memory,notify,scheduler,service,
network,backup,cost,auth,observability}-*` sibling crate provides one.

Every trait extends [`HasDna`]: there are no anonymous impls. Every
registered impl carries a serial that traces parent → child via
[`HasGenealogy`].

See `~/Projects/keisei-marketplace/spec/DNA-CONVENTION.md` for the
universal serial format and `spec/CONFIG-REFERENCE.md` for the
per-trait configuration surface.

## Related

- parent: `kei-runtime-core/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
