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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
