---
title: keys.rs
path: kei-store/src/s3_cloud/keys.rs
dna_hash: sha256:752ef700509564ce
language: rust
size_loc: 40
generated: by-keidocs
---

# kei-store/src/s3_cloud/keys.rs

Key-path helpers for the S3 cloud backend.

v0.22 Track B: the actual logic moved one level up into
`crate::async_backend` (shared by every future cloud backend — GCS,
Azure Blob, Bunny, etc.). This module now re-exports the helpers for
backward source-compat and keeps the unit tests green.

## Related

- parent: `kei-store/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
