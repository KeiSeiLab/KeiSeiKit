---
title: sha.rs
path: kei-hibernate/src/sha.rs
dna_hash: sha256:8f43190102e5f2da
language: rust
size_loc: 41
generated: by-keidocs
---

# kei-hibernate/src/sha.rs

SHA-256 helper — stream file bytes without loading whole payload.
Used by exporter (manifest hashes) and importer (post-extract verify).

## Public API

- `pub fn hex` — Hex-encode a 32-byte digest. Lowercase, no delimiters.
- `pub fn hash_bytes` — Hash bytes already in memory. Small helper for tests + manifest string.
- `pub fn hash_file` — Stream a file through Sha256 in 64KB chunks. Error surfaces raw io;

## Related

- parent: `kei-hibernate/Cargo.toml`
- imports: sha2, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
