---
title: error.rs
path: kei-auth-google/src/error.rs
dna_hash: sha256:4682868b6f776dad
language: rust
size_loc: 97
generated: by-keidocs
---

# kei-auth-google/src/error.rs

Error types for `kei-auth-google`. Maps cleanly into
[`kei_runtime_core::Error`] so the provider can fulfil
[`kei_runtime_core::traits::auth::AuthProvider`].

## Public API

- Transport-level reqwest failure (connect, TLS, decode).
- Google API returned a non-success status with a body we surface
- Caller passed a non-OAuthCode challenge OR omitted the `state` ⇄ code
- Userinfo lookup returned 404 or the requested resource is absent.
- DNA composition failed (only possible if scope/body inputs violate
- Underlying serde decode failure on a JSON body Google returned.
- Configuration mismatch (env var unset, both URLs absent, etc.).
- Google account email is not verified — refusing authentication.
- `id_token.sub` from the token endpoint disagrees with
- `id_token` was syntactically malformed (not three segments,

## Related

- parent: `kei-auth-google/Cargo.toml`
- imports: thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
