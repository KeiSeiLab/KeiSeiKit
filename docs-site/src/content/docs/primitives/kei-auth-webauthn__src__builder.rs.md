---
title: builder.rs
path: kei-auth-webauthn/src/builder.rs
dna_hash: sha256:795a2ab4b28279a5
language: rust
size_loc: 28
generated: by-keidocs
---

# kei-auth-webauthn/src/builder.rs

[`build_webauthn`] — thin wrapper around `webauthn_rs::WebauthnBuilder`.

Centralises relying-party (RP) configuration so [`crate::WebauthnProvider`]
constructors stay short. No logic of our own — defaults come from
webauthn-rs upstream (FIDO MDS optional, ResidentKey by default).

## Public API

- `pub fn build_webauthn` — Build a [`Webauthn`] instance for a given relying party.

## Related

- parent: `kei-auth-webauthn/Cargo.toml`
- imports: crate, url, webauthn_rs

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
