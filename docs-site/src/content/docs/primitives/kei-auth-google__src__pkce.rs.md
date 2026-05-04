---
title: pkce.rs
path: kei-auth-google/src/pkce.rs
dna_hash: sha256:a77f3c6ac4ecba68
language: rust
size_loc: 51
generated: by-keidocs
---

# kei-auth-google/src/pkce.rs

PKCE (RFC 7636) helpers and URL percent-encoder shared by the Google
auth provider.

## Public API

- `pub fn pkce_challenge` — Compute PKCE `code_challenge` = `BASE64URL-no-pad(SHA256(verifier))`.
- Percent-encode a string per RFC 3986 §2.1 (only unreserved chars pass).

## Related

- parent: `kei-auth-google/Cargo.toml`
- imports: base64, sha2

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
