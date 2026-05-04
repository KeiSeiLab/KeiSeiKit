---
title: routes_auth.rs
path: kei-cortex/src/routes_auth.rs
dna_hash: sha256:f89909e12d393c0a
language: rust
size_loc: 69
generated: by-keidocs
---

# kei-cortex/src/routes_auth.rs

Bearer-token middleware for the cortex API router.

Separated from `routes.rs` so each file stays under 200 LOC.

## Public API

- Bearer-token middleware.
- Pull `<token>` from `Authorization: Bearer <token>` if present.
- Pull `<token>` from `Sec-WebSocket-Protocol: bearer, <token>`. The
- Compare expected vs. supplied; on match, call `next`.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
