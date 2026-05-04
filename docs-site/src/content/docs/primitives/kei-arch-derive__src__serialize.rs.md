---
title: serialize.rs
path: kei-arch-derive/src/serialize.rs
dna_hash: sha256:b776bae351112dd1
language: rust
size_loc: 97
generated: by-keidocs
---

# kei-arch-derive/src/serialize.rs

Inline-evidence TOML rendering. Produces the single-line inline-table
shape used by hand-written `arch/PLAN.toml` so the auto-generated and
hand-edited files round-trip through `kei_arch_map::schema::load`.

Constructor Pattern: this cube ONLY serialises one `EvidenceClaim` to
one TOML inline-table line. Bulk plan rendering lives in `emit.rs`.

## Public API

- `pub fn inline_evidence` — Render an `EvidenceClaim` as a single TOML inline-table string.
- `pub fn quote` — TOML basic-string quoting: backslash and double-quote escape, wrap in `"`.

## Related

- parent: `kei-arch-derive/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
