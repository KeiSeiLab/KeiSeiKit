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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
