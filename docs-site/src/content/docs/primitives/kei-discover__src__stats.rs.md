---
title: stats.rs
path: kei-discover/src/stats.rs
dna_hash: sha256:0fb1e4b71ba8b2f4
language: rust
size_loc: 31
generated: by-keidocs
---

# kei-discover/src/stats.rs

`stats` — aggregate counts (total / installed / available).

One-row SELECT with conditional SUMs. `available` equals
`total - installed`, kept as an explicit field so CLI users don't
have to subtract.

## Related

- parent: `kei-discover/Cargo.toml`
- imports: crate, rusqlite, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
