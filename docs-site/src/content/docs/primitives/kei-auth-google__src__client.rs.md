---
title: client.rs
path: kei-auth-google/src/client.rs
dna_hash: sha256:9fa45e49ca109a70
language: rust
size_loc: 169
generated: by-keidocs
---

# kei-auth-google/src/client.rs

Thin async client for Google OAuth 2.0 token + OIDC userinfo endpoints.

Two HTTP calls cover the verify path:
1. `POST {token_url}` (x-www-form-urlencoded) → access_token + id_token
2. `GET {userinfo_url}` with `Authorization: Bearer <access_token>`

## Public API

- `pub const DEFAULT_AUTH_URL` — Authorization endpoint — used only by [`super::provider::GoogleAuthProvider::build_auth_url`].
- Token-endpoint response (RFC 6749 §5.1 + OIDC `id_token`).
- Userinfo response (OIDC core §5.3.2 — only the fields we surface).
- OIDC `email_verified` boolean. Defaults to `false` when the
- Async client wrapping the two relevant Google endpoints.
- Wrapped in `SecretString` so it prints as `<redacted>` in logs.
- `pub fn from_env` — Build from `GOOGLE_OAUTH_CLIENT_ID`, `GOOGLE_OAUTH_CLIENT_SECRET`,
- `pub fn with_urls` — Explicit-URL constructor — used by `wiremock` and any caller that
- `POST {token_url}` (x-www-form-urlencoded) →
- `GET {userinfo_url}` with `Authorization: Bearer <access_token>`.
- `pub fn client_id` — Borrow `client_id` (used by `build_auth_url`).
- `pub fn redirect_uri` — Borrow `redirect_uri` (used by `build_auth_url`).

## Related

- parent: `kei-auth-google/Cargo.toml`
- imports: crate, kei_runtime_core, reqwest, serde, std

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
