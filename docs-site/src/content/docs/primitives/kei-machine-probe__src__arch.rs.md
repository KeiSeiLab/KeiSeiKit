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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
