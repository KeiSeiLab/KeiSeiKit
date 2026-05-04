---
title: error.rs
path: kei-auth-magiclink/src/error.rs
dna_hash: sha256:c5e48487c0f1d35b
language: rust
size_loc: 51
generated: by-keidocs
---

# kei-auth-magiclink/src/error.rs

Error type for kei-auth-magiclink.

Maps cleanly into `kei_runtime_core::Error::Auth(String)` so the
[`AuthProvider`] trait surface stays in the substrate's error space.

## Public API

- Token does not split into the expected three `.`-separated parts,
- Token's expiry timestamp is at or before `now_unix_ms`.
- HMAC tag does not match the recomputed tag (constant-time compare).
- `MAGICLINK_HMAC_KEY` env var is missing, empty, undecodable,
- DNA build / parse error from `kei_runtime_core::dna`.
- Operation is not supported by this provider — surfaced when a caller

## Related

- parent: `kei-auth-magiclink/Cargo.toml`
- imports: thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
