---
title: arch.rs
path: kei-machine-probe/src/arch.rs
dna_hash: sha256:ea9f48ddfecd3356
language: rust
size_loc: 77
generated: by-keidocs
---

# kei-machine-probe/src/arch.rs

CPU / arch detection via `sysctl`.

Three sysctl reads:
`sysctl -n hw.model`                 → Mac model id (e.g. `Mac14,7`)
`sysctl -n machdep.cpu.brand_string` → marketing string (`Apple M2 Pro`)
`sysctl -n hw.optional.arm64`        → 1 ⇒ Apple Silicon, 0 ⇒ Intel
`sysctl -n hw.ncpu`                  → physical+logical core count

Mapping: `family` from the arm64 flag, `variant` parsed from the
brand string. Anything we can't classify falls into
`AppleVariant::Unknown` rather than panicking.

## Public API

- Match `brand` (e.g. "Apple M2 Pro") to the closest `AppleVariant`.

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: crate

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
