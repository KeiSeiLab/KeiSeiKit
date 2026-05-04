---
title: lib.rs
path: kei-auth-magiclink/src/lib.rs
dna_hash: sha256:1450feccf60e3386
language: rust
size_loc: 69
generated: by-keidocs
---

# kei-auth-magiclink/src/lib.rs

kei-auth-magiclink — passwordless email magic-link `AuthProvider`.

Wave 7 atomar substrate primitive. Sibling of:

- `kei-auth`           — multi-tenant SQLite token store
- `kei-auth-google`    — Google OAuth 2.0 + OIDC
- `kei-auth-{github,microsoft,apple}` (forthcoming)

## What this is

A self-contained `AuthProvider` impl that issues and verifies
HMAC-SHA256-signed magic-link tokens. **Stateless**: no DB, no Redis,
no shared secret beyond a single 32+ byte HMAC key.

## Token wire format

```text
<email_b64url>.<expires_unix_ms_b64url>.<hmac_sha256_b64url>
```

All parts are URL-safe base64, no padding. See [`token`].

## Configuration

Two environment variables (read by [`MagicLinkProvider::from_env`]):

- `MAGICLINK_HMAC_KEY` — 32+ bytes after decoding. Hex if 64 ASCII chars,
otherwise standard base64. **Required.**
- `MAGICLINK_TTL_SECS` — i64 decimal, default 900 (15 minutes). Optional.

## Trait quirk to know

[`AuthChallenge::MagicLink`] only carries an `email: String`. We use it
two ways, by convention:

- On `issue_challenge` it is the user's email.
- On `verify` it is the FULL token string (the one in `?token=…`).

Callers wire their HTTP handler accordingly. The alternative —
extending the trait with a `MagicLinkVerify { token }` variant — is a
bigger surface change and is left for a future revision of
`kei-runtime-core`.

## DNA

Every [`MagicLinkProvider`] owns the literal DNA:

```text
DnaBuilder::new("primitive")
.caps(["PR", "AP", "ML"])
.scope("keiseikit.dev/primitives/kei-auth-magiclink")
.body(b"magiclink-v1")
.build()?
```

Each verified session gets its own short-lived `session` DNA derived
from the user's email body — see [`provider`].

## Related

- parent: `kei-auth-magiclink/Cargo.toml`

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
