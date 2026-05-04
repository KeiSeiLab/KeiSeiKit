---
title: parsed.rs
path: kei-dna-index/src/parsed.rs
dna_hash: sha256:29e1eafe55e8962b
language: rust
size_loc: 60
generated: by-keidocs
---

# kei-dna-index/src/parsed.rs

DNA parser — thin wrapper over `kei_shared::dna`.

Format: `<role>::<caps>::<sha8-scope>::<sha8-body>-<hex8-nonce>`
Example: `edit-local::NG-FW-FD-CP-CG-TG-ND-RF::5435F821::AC73A6A3-e9bf468d`

Wire-format SSoT lives in `kei_shared::dna`. This module re-exports
`ParsedDna` and exposes `split_dna` that maps `kei_shared::DnaError`
into `crate::Error::MalformedDna` so callers keep a single error type.

## Public API

- `pub fn split_dna` — Parse a DNA string into its five fields. Hex widths are validated.

## Related

- parent: `kei-dna-index/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
