---
title: dna.rs
path: keidocs/src/dna.rs
dna_hash: sha256:7dee204d408d3532
language: rust
size_loc: 70
generated: by-keidocs
---

# keidocs/src/dna.rs

DNA hash — sha256 of (path + sorted deps + content), truncated 16 hex chars.

Stable: same inputs → same hash. Sorting deps removes spurious diff noise.

## Public API

- `pub fn compute_dna` — Compute a deterministic content-addressable id for a source file.

## Related

- parent: `keidocs/Cargo.toml`
- imports: sha2

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
