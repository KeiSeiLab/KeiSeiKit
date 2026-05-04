---
title: provider.rs
path: kei-auth-webauthn/src/provider.rs
dna_hash: sha256:702dc0ed6d1fc9bd
language: rust
size_loc: 171
generated: by-keidocs
---

# kei-auth-webauthn/src/provider.rs

[`WebauthnProvider`] — DNA-bearing [`AuthProvider`] impl that wraps
a configured `webauthn_rs::Webauthn` instance.

See `crate` doc for the trait-extension convention (`AuthChallenge`
has no `Webauthn` variant; the trait methods point callers at the
explicit ceremony helpers below).

## Public API

- `pub struct WebauthnProvider` — WebAuthn passkey AuthProvider. Stateless — owns no session store and no
- `pub fn new` — Construct a new provider with no parent DNA.
- `pub fn with_parent` — Construct with an explicit parent DNA (for genealogy attribution).
- `pub fn webauthn` — Borrow the configured `Webauthn` instance (escape hatch for
- `pub fn start_registration` — Begin a passkey-registration ceremony.
- `pub fn finish_registration` — Complete a passkey-registration ceremony.
- `pub fn start_authentication` — Begin a passkey-authentication ceremony.
- `pub fn finish_authentication` — Complete a passkey-authentication ceremony.
- Trait-shaped no-op. WebAuthn ceremonies do not fit
- Trait-shaped error. See [`Self::issue_challenge`] — drive
- Passkey revocation is operator-managed: delete the [`Passkey`]

## Related

- parent: `kei-auth-webauthn/Cargo.toml`
- imports: async_trait, crate, kei_runtime_core, uuid, webauthn_rs

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
