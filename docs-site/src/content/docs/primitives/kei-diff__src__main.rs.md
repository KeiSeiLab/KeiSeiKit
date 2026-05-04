---
title: main.rs
path: kei-diff/src/main.rs
dna_hash: sha256:dc4acd40f370d196
language: rust
size_loc: 80
generated: by-keidocs
---

# kei-diff/src/main.rs

kei-diff CLI.

Usage:
kei-diff diff  --old <path> --new <path>       # prints RFC 6902 patch
kei-diff apply --base <path> --patch <path>    # prints result document

No external arg-parser dep — this is a two-verb tool with fixed flag sets,
hand-rolling keeps the crate zero-dep beyond serde/serde_json.

## Related

- parent: `kei-diff/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
