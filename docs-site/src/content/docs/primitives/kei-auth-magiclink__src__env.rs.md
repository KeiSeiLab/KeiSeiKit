---
title: env.rs
path: kei-auth-magiclink/src/env.rs
dna_hash: sha256:9a325e19c71926b9
language: rust
size_loc: 92
generated: by-keidocs
---

# kei-auth-magiclink/src/env.rs

Environment + key-decoding helpers for `MagicLinkProvider::from_env`.

Kept in its own cube so [`provider`](crate::provider) stays under the
Constructor-Pattern 200-LOC limit. Pure functions, no trait surface.

## Public API

- `pub fn read_env` — Read `MAGICLINK_HMAC_KEY` and `MAGICLINK_TTL_SECS` from the environment.
- `pub fn decode_key` — Decode a key string. 64 ASCII hex chars → hex; else standard base64.

## Related

- parent: `kei-auth-magiclink/Cargo.toml`
- imports: base64, crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
