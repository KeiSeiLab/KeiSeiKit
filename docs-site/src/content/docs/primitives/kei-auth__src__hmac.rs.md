---
title: hmac.rs
path: kei-auth/src/hmac.rs
dna_hash: sha256:898261f91d2af934
language: rust
size_loc: 25
generated: by-keidocs
---

# kei-auth/src/hmac.rs

HMAC-SHA256 signer for token bodies.

## Public API

- `pub fn sign` — Sign `body` with `key`. Returns URL-safe base64 MAC.
- `pub fn verify` — Verify `body` against MAC. Returns Err if mismatch.

## Related

- parent: `kei-auth/Cargo.toml`
- imports: anyhow, base64, sha2

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
