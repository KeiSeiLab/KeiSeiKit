---
title: extract.rs
path: kei-router/src/extract.rs
dna_hash: sha256:0936d26eb5acc1f5
language: rust
size_loc: 167
generated: by-keidocs
---

# kei-router/src/extract.rs

Param extraction — regex scans the raw query for path / limit / id / URI / KV.

Ported from LBM pkg/keirouter/extract.go.

## Public API

- `pub fn extract_params` — Parse a raw NL query into structured [`Extracted`] params.

## Related

- parent: `kei-router/Cargo.toml`
- imports: regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
