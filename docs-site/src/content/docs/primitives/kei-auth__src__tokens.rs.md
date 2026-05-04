---
title: tokens.rs
path: kei-auth/src/tokens.rs
dna_hash: sha256:8841730d629bbaa7
language: rust
size_loc: 126
generated: by-keidocs
---

# kei-auth/src/tokens.rs

Token issue / verify / revoke.

Token layout (URL-safe, no padding):
`<b64(payload_json)>.<b64(hmac)>`
Payload contains {tid, user_id, project, scope, expires_at}.
The db keeps sha256(token) to support revocation and lookup.

## Public API

- `pub fn issue` — Issue a new token. The returned string is the ONLY copy — DB stores only its sha256.
- `pub fn verify` — Verify a token: signature valid, not revoked, not expired, returns identity + scope.
- `pub fn revoke` — Mark a token as revoked. Returns number of rows affected (0 = unknown).

## Related

- parent: `kei-auth/Cargo.toml`
- imports: anyhow, base64, chrono, crate, rand, rusqlite, serde, sha2, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
