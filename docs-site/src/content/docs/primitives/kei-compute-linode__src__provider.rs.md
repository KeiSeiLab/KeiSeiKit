---
title: provider.rs
path: kei-compute-linode/src/provider.rs
dna_hash: sha256:106591f5f1b5ba3e
language: rust
size_loc: 279
generated: by-keidocs
---

# kei-compute-linode/src/provider.rs

`LinodeCompute` — `ComputeProvider` impl over `LinodeClient`.

- DNA: `primitive::PR-AP-LN::<scope8>::<body8>-<nonce>` for the
provider itself; `vm-managed::LN-<TYPE_UPPER>-<REGION_UPPER>::…`
for each provisioned VM.
- Tier policy: only the slugs in [`TIERS`] are accepted.
- Cost: USD micro-cents/hour, on-demand pricing as of 2026-04
(RULE 0.4: pricing is per Linode public pricing page; values
carried as constants inline so this crate is self-contained).

## Public API

- `pub const TIERS` — Allowed Linode tier slugs (subset relevant to the kit's workloads).
- `pub struct LinodeCompute` — Provider impl. Holds the HTTP client + its DNA serial.
- `pub fn new` — Build with explicit client + default image (e.g. `linode/debian12`).
- `pub fn map_status` — Map Linode `status` strings to the trait's `VmStatus`.

## Related

- parent: `kei-compute-linode/Cargo.toml`
- imports: async_trait, base64, crate, kei_runtime_core, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
