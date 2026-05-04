---
title: id_token.rs
path: kei-auth-google/src/id_token.rs
dna_hash: sha256:67f5d41470a229e4
language: rust
size_loc: 104
generated: by-keidocs
---

# kei-auth-google/src/id_token.rs

ID-token claim extraction for Google OIDC.

**Scope (deliberate, narrow).** This module decodes the *claims*
payload of a JWT — the middle base64url segment — and surfaces the
`sub` field. It does **not** verify the JWT signature against
Google's JWKS. Signature verification is a follow-up (load JWKS
over HTTPS, cache by `kid`, run RS256/ES256). Until then, the
`id_token.sub` is treated as a defence-in-depth cross-check
against the userinfo `sub` (the token came from a TLS-validated
token endpoint, but a malicious userinfo response could still
ship a different `sub` if the access token leaked).

See RFC 7519 §3 (JWT compact serialization) and OIDC Core §2
(id_token claims).

[VERIFIED: https://datatracker.ietf.org/doc/html/rfc7519]

## Public API

- Minimal projection of the OIDC id_token claims payload.
- Stable Google account identifier; matches userinfo `sub`.
- `pub fn extract_sub` — Parse the **claims** segment of a JWT and decode `sub`.
- Pull the middle (claims) segment of a JWT compact serialization.
- base64url-no-pad decode (RFC 7515 §2). Tolerant of optional padding.

## Related

- parent: `kei-auth-google/Cargo.toml`
- imports: base64, crate, serde

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
