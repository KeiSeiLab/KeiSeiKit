---
title: token.rs
path: kei-auth-magiclink/src/token.rs
dna_hash: sha256:8bd2bd5c13a6ea84
language: rust
size_loc: 158
generated: by-keidocs
---

# kei-auth-magiclink/src/token.rs

Magic-link token codec.

Wire format (URL-safe base64, no padding, three `.`-separated parts):

```text
<email_b64url>.<expires_unix_ms_b64url>.<hmac_sha256_b64url>
```

- `email_b64url`     — UTF-8 bytes of the email, base64url-encoded.
- `expires_unix_ms`  — decimal ASCII of an i64 unix-ms timestamp,
base64url-encoded as bytes (lets us keep the
same alphabet end-to-end).
- `hmac_sha256`      — 32-byte HMAC-SHA256 of the literal ASCII
`<email_b64url>.<expires_unix_ms_b64url>`,
base64url-encoded.

Verification is stateless: no DB lookup. Revocation, if needed, is
the caller's responsibility (e.g. a parallel deny-list keyed on the
token's first two parts).

## Public API

- `pub fn build_token` — Build a signed magic-link token.
- `pub fn parse_token` — Parse, verify HMAC, and check expiry.

## Related

- parent: `kei-auth-magiclink/Cargo.toml`
- imports: base64, crate, hmac, sha2, subtle

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
