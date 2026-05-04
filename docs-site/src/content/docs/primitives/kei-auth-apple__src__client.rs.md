---
title: client.rs
path: kei-auth-apple/src/client.rs
dna_hash: sha256:ed094c325bc7bfb8
language: rust
size_loc: 139
generated: by-keidocs
---

# kei-auth-apple/src/client.rs

Thin async OAuth client for Apple Sign-In code exchange.

Implements only the `POST /auth/token` step (RFC 6749 §4.1.3
Authorization Code grant) against the Apple ID endpoint. Apple's
`client_secret` is itself an ES256-signed JWT — this cube does NOT
sign it; the caller MUST supply a pre-built JWT.

## Public API

- `pub const DEFAULT_AUTHORIZE_URL` — Default authorization endpoint (browser-facing redirect target).
- `pub const DEFAULT_TOKEN_URL` — Default token endpoint (server-side code exchange POST).
- `pub const DEFAULT_TIMEOUT_SECS` — Per-request timeout.
- Apple `/auth/token` response shape (RFC 6749 + Apple-specific fields).
- REST client for the Apple `/auth/token` endpoint. Cheap to clone.
- Wrapped in `SecretString` so logs never reveal the JWT.
- `pub fn with_url` — Build with explicit values (use [`DEFAULT_TOKEN_URL`] in prod).
- `pub fn from_env` — Read all three required values from env, default token URL.
- `pub fn client_id` — Borrow `client_id` (used by `build_auth_url`).
- `pub fn redirect_uri` — Borrow `redirect_uri` (used by `build_auth_url`).
- POST application/x-www-form-urlencoded body to `/auth/token`.

## Related

- parent: `kei-auth-apple/Cargo.toml`
- imports: crate, kei_runtime_core, reqwest, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
