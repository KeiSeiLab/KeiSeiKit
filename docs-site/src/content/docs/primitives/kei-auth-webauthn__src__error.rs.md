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
