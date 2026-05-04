---
title: provider.rs
path: kei-auth-apple/src/provider.rs
dna_hash: sha256:5bffb4e083cdb88b
language: rust
size_loc: 187
generated: by-keidocs
---

# kei-auth-apple/src/provider.rs

[`AppleAuthProvider`] — DNA-bearing [`AuthProvider`] impl for Sign in
with Apple.

`user_id` on the resulting [`AuthSession`] is taken from the JWT `sub`
claim (stable Apple user id). The `verify()` method performs ES256
signature verification via [`verify_id_token`] against the caller-supplied
JWKS JSON.

## Public API

- DNA-bearing Apple Sign-In auth provider.
- Raw JWKS JSON from `https://appleid.apple.com/auth/keys`.
- `pub fn new` — Build a provider with a fresh DNA serial.
- `pub fn build_auth_url` — Build an authorization URL for the Apple Sign-In redirect.
- Constant-time CSRF state comparison. Returns `CsrfStateMismatch` on
- `pub fn pkce_challenge` — Compute the PKCE `code_challenge` from a plain `code_verifier`.

## Related

- parent: `kei-auth-apple/Cargo.toml`
- imports: async_trait, base64, crate, kei_runtime_core, sha2, std, subtle

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
