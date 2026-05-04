---
title: provider.rs
path: kei-compute-vultr/src/provider.rs
dna_hash: sha256:a77a386ba316f7e0
language: rust
size_loc: 296
generated: by-keidocs
---

# kei-compute-vultr/src/provider.rs

`VultrCompute` — `ComputeProvider` impl for Vultr Cloud v2.

DNA caps tag is `VL` (per spec/DNA-CONVENTION.md). Tier table covers
the 6 cheapest single-CPU plans (vc2 / vhf, 1-4 GB) — extendable as
needed. Cost numbers are µ-cents per hour, converted from Vultr's
published $/month rate divided by 730 hours/month and rounded.

## Public API

- Validated Vultr tier id list. Anything else fails `validate_tier`.
- Optional Vultr SSH key id (already uploaded out-of-band).
- OS id used at create time when caller doesn't override (Debian 12 = 2136).
- `pub fn from_env` — Construct from `VULTR_API_KEY` (env). Errors if missing.

## Related

- parent: `kei-compute-vultr/Cargo.toml`
- imports: async_trait, crate, kei_runtime_core

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
