---
title: factory.rs
path: kei-store/src/factory.rs
dna_hash: sha256:855b2c3b11a55ba4
language: rust
size_loc: 76
generated: by-keidocs
---

# kei-store/src/factory.rs

Factory — construct a `Box<dyn MemoryStore>` from a Config.

v0.14.1: the S3 backend is gated behind `KEI_STORE_ALLOW_S3_STUB=1`
because the default build has no real S3 push — it's a local-manifest
stub. Previous behaviour silently stored data locally, confusing users
who thought their traces were uploaded.

v0.21.0: when the crate is built with `--features s3` AND
`s3.bucket` is configured, the real `S3CloudStore` is used (no
KEI_STORE_ALLOW_S3_STUB gate needed — data really is uploaded).
The stub path remains available for users who don't want the AWS SDK
in their binary: omit `s3.bucket` and set the stub opt-in env.

## Related

- parent: `kei-store/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
