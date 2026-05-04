---
title: jwt.rs
path: kei-auth-apple/src/jwt.rs
dna_hash: sha256:1441f3999a28d525
language: rust
size_loc: 164
generated: by-keidocs
---

# kei-auth-apple/src/jwt.rs

Apple id_token verification — ES256 signature check against Apple JWKS.

Production path: [`verify_id_token`] — verifies signature, validates
standard claims (`iss`, `aud`, `exp`, `iat`).

Test-only path: [`decode_id_token_unverified`] — available only under
`#[cfg(test)]`; never present in production builds.

## Public API

- `pub fn verify_id_token` — Verify an Apple id_token against the provided JWKS JSON, checking:
- Decode the claims segment of a JWT WITHOUT verifying the signature.

## Related

- parent: `kei-auth-apple/Cargo.toml`
- imports: base64, crate, jsonwebtoken, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
