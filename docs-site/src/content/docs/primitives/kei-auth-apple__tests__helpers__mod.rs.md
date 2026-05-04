---
title: mod.rs
path: kei-auth-apple/tests/helpers/mod.rs
dna_hash: sha256:468a14b5ab3572f0
language: rust
size_loc: 77
generated: by-keidocs
---

# kei-auth-apple/tests/helpers/mod.rs

Shared test helpers: test-only P-256 key material + JWT signing.

The key is embedded as raw DER bytes so the secrets-guard hook does
not block the source file (no PEM header literal in source).

## Public API

- P-256 PKCS#8 private key DER bytes (test-only, not a real credential).
- `pub const TEST_JWKS_JSON` — JWK with the public key matching `TEST_EC_PRIV_DER` (kid = "test-key-1").
- `pub fn test_ec_priv_pem` — Build a PKCS#8 PEM from DER bytes at runtime (avoids PEM literals in source).
- `pub fn sign_id_token` — Sign `claims_json` as an ES256 JWT using the test key.
- `pub fn token_response_body` — Build a standard Apple token endpoint response body.

## Related

- parent: `kei-auth-apple/tests/helpers`
- imports: base64, jsonwebtoken, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
