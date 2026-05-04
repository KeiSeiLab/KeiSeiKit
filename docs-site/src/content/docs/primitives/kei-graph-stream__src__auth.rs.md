---
title: auth.rs
path: kei-graph-stream/src/auth.rs
dna_hash: sha256:ae09786f022cfe5f
language: rust
size_loc: 142
generated: by-keidocs
---

# kei-graph-stream/src/auth.rs

Bearer token + Origin validation for WebSocket upgrades.

Token is loaded from `~/.keisei/cortex.token` (same file as kei-cortex).
Origin allowlist: localhost and 127.0.0.1 on any port, plus the literal
string "null" (used by some browsers for file:// origins).

## Public API

- Error returned when auth fails.
- `pub fn load_expected_token` — Load the expected bearer token from `~/.keisei/cortex.token`.
- `pub fn extract_bearer` — Extract the bearer token from `Sec-WebSocket-Protocol: bearer,<token>`.
- `pub fn validate_origin` — Validate `Origin` is in the local allowlist.
- `pub fn tokens_match` — Constant-time comparison (length-gated xor fold).

## Related

- parent: `kei-graph-stream/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
