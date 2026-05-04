---
title: gpu.rs
path: kei-machine-probe/src/gpu.rs
dna_hash: sha256:1ab3e0de8466f8fa
language: rust
size_loc: 80
generated: by-keidocs
---

# kei-machine-probe/src/gpu.rs

GPU detection via `system_profiler SPDisplaysDataType -json`.

Apple Silicon machines report a single integrated GPU under
`SPDisplaysDataType[0]` with `sppci_model = "Apple M2 Pro"` (etc) and
`sppci_cores = "19"`. Intel iMacs / MacBook Pros surface either an
Intel-integrated entry or a discrete AMD/NVIDIA entry (sometimes both;
we pick the discrete one as the more capable).

## Public API

- Apple integrated > Discrete > IntelIntegrated > None ordering.

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: crate, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
