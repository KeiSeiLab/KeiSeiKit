---
title: auth.rs
path: kei-cortex/src/routes/openai/auth.rs
dna_hash: sha256:cf8fa90dd43c0031
language: rust
size_loc: 112
generated: by-keidocs
---

# kei-cortex/src/routes/openai/auth.rs

Bearer-token middleware for the OpenAI-compatible `/v1/*` surface.

Two acceptance modes:
1. `KEI_API_KEY` env var is set → require `Authorization: Bearer <key>`
and compare in constant time via `subtle::ConstantTimeEq`.
2. `KEI_API_KEY` unset → allow only requests whose source IP is the
loopback address (127.0.0.1 / ::1). This matches Hermes' default
and prevents accidentally exposing a tokenless endpoint on the LAN.

## Public API

- Env var name read at request time. Reading per-request lets the daemon
- Tower middleware. Reject 401 on missing/invalid key when one is set;
- Constant-time bearer-token comparison. Returns `Unauthorized` on
- Constant-time byte-slice equality. Returns false fast on length
- Pull `<token>` from `Authorization: Bearer <token>`.
- When no API key is configured, restrict to loopback callers. Missing

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
