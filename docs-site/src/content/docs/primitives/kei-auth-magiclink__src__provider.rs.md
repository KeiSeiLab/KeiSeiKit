---
title: provider.rs
path: kei-auth-magiclink/src/provider.rs
dna_hash: sha256:541d67763afc1995
language: rust
size_loc: 180
generated: by-keidocs
---

# kei-auth-magiclink/src/provider.rs

[`MagicLinkProvider`] — passwordless `AuthProvider` impl.

Stateless HMAC-signed tokens. No DB. The provider is a value-typed
cube: `dna`, `parent`, `hmac_key`, `ttl_secs`. Construct via
[`MagicLinkProvider::new`] (explicit) or [`MagicLinkProvider::from_env`]
(reads `MAGICLINK_HMAC_KEY` and `MAGICLINK_TTL_SECS`).

## Trait convention quirk

[`AuthChallenge::MagicLink`] only carries an `email` field. Two paths use it:

- [`MagicLinkProvider::issue_challenge`] — `email` is the user's address.
The provider does NOT send the email itself (no dependency on
`kei-notify-*`); callers build the URL via
[`MagicLinkProvider::build_magic_url`] and dispatch through their own
notify channel.
- [`MagicLinkProvider::verify`] — `email` MUST be the FULL token string
returned in the URL's `?token=…` query param. Callers wire the web
handler to slot the token into the `email` field. This is the minimum
change consistent with the trait surface as of v0.1; a future
`AuthChallenge::MagicLinkVerify { token }` variant would be cleaner.

## Public API

- Stateless HMAC-SHA256 magic-link provider.
- `pub fn new` — Construct with an explicit parent DNA, key bytes, and TTL.
- `pub fn from_env` — Construct from environment. See [`crate::env`] for variable names.
- `pub fn build_magic_url` — Build the URL the caller emails to the user.
- `pub fn ttl_secs` — Configured TTL in seconds.

## Related

- parent: `kei-auth-magiclink/Cargo.toml`
- imports: async_trait, crate, kei_runtime_core, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
