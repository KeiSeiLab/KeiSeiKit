---
title: lib.rs
path: kei-auth-apple/src/lib.rs
dna_hash: sha256:d59dc03deb743f6e
language: rust
size_loc: 41
generated: by-keidocs
---

# kei-auth-apple/src/lib.rs

kei-auth-apple — Sign in with Apple impl of [`kei_runtime_core::AuthProvider`].

Layout:
- [`error`]: local `Error`/`Result` mapping into the runtime-core error.
- [`client`]: thin async OAuth code-exchange client (mockable URL).
- [`jwt`]: unverified base64-url id_token claim decoder.
- [`provider`]: [`AppleAuthProvider`] — DNA-bearing trait impl.

Endpoints:
- Authorize: `https://appleid.apple.com/auth/authorize`
- Token:     `https://appleid.apple.com/auth/token`

Auth required (env):
- `APPLE_OAUTH_CLIENT_ID`        — services-id reverse domain (e.g. `com.example.web`).
- `APPLE_CLIENT_SECRET_JWT`      — pre-built ES256 client_secret JWT.
- `APPLE_OAUTH_REDIRECT_URI`     — registered redirect URI.

KNOWN LIMITATION (v0.1):
- Apple requires `client_secret` to be an ES256-signed JWT over
`(team_id, bundle_id, key_id)`. Producing that JWT is OUT OF SCOPE for
this atomic cube; the caller MUST supply a pre-built JWT in
`APPLE_CLIENT_SECRET_JWT`. Signing the JWT will live in a future sister
crate `kei-auth-apple-jwt`.
- The id_token returned by Apple is a JWT signed with Apple's JWKS. v0.1
decodes the claims segment WITHOUT signature verification. Full JWKS
validation also lives in the future `kei-auth-apple-jwt` cube.

## Related

- parent: `kei-auth-apple/Cargo.toml`

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
