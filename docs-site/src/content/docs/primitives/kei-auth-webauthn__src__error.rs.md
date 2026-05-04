---
title: error.rs
path: kei-auth-webauthn/src/error.rs
dna_hash: sha256:2e0d90bb076167fb
language: rust
size_loc: 56
generated: by-keidocs
---

# kei-auth-webauthn/src/error.rs

Error types for `kei-auth-webauthn`. Maps cleanly into
[`kei_runtime_core::Error`] so [`crate::WebauthnProvider`] can fulfil
[`kei_runtime_core::traits::auth::AuthProvider`].

## Public API

- Underlying webauthn-rs ceremony failure (validation, parse, crypto).
- Invalid relying-party origin URL (must be parseable by `url::Url`).
- DNA composition failed (only possible if the literal scope/body
- Caller attempted to drive a WebAuthn ceremony through the

## Related

- parent: `kei-auth-webauthn/Cargo.toml`
- imports: thiserror, webauthn_rs, {s}

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
