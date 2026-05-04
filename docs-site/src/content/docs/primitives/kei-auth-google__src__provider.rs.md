---
title: provider.rs
path: kei-auth-google/src/provider.rs
dna_hash: sha256:b662aa97cd4fa205
language: rust
size_loc: 181
generated: by-keidocs
---

# kei-auth-google/src/provider.rs

[`GoogleAuthProvider`] — `AuthProvider` impl over Google OAuth 2.0 +
OIDC userinfo. Builds an [`AuthSession`] whose `user_id` is the OIDC
`sub` claim (Google's stable account-id; emails can change).

## Security model

- **`email_verified` gate.** `verify()` rejects any userinfo response
with `email_verified == false`. CVE-2023-7028 class: Google
Workspace tenants can mint accounts with arbitrary unverified
email aliases. Trusting `email` without the verified flag is
account-takeover-equivalent.
- **`sub` as user_id.** `info.email` is exposed only as metadata;
the primary identifier is `info.sub` (Google's `255-byte stable
account identifier`). Email is mutable; sub is not.
- **`id_token.sub` cross-check.** When the token endpoint returns
an `id_token`, we decode its claims and verify `sub` matches the
userinfo response. Defence in depth against a forged userinfo.
*Note:* JWT signature verification (RS256 against Google's JWKS)
is a follow-up — the current code parses claims only.

`provider_name = "google"`. `is_passwordless = true`.

## Public API

- `pub const DEFAULT_SCOPES` — Default scope set: OIDC profile + email.
- `pub struct GoogleAuthProvider` — `AuthProvider` for Google OAuth 2.0.
- `pub fn new` — Build a provider over a pre-configured client.
- `pub fn build_auth_url` — Build the redirect URL for the Google OAuth 2.0 consent screen.
- `pub fn client` — Borrow the underlying client.

## Related

- parent: `kei-auth-google/Cargo.toml`
- imports: async_trait, crate, kei_runtime_core, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
