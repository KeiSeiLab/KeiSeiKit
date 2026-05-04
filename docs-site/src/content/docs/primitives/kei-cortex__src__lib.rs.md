---
title: lib.rs
path: kei-cortex/src/lib.rs
dna_hash: sha256:f1a06c244e61951f
language: rust
size_loc: 43
generated: by-keidocs
---

# kei-cortex/src/lib.rs

kei-cortex — local HTTP daemon exposing cortex state for UI consumption.

Constructor Pattern: one module = one responsibility. This crate wires up:
`auth` (bearer-token lifecycle), `config` (CLI/env binding), `error`
(typed JSON responses), `state` (shared handler state), `routes` (router
+ middleware), `handlers` (endpoint implementations).

The daemon is intended to serve a single user on `127.0.0.1:9797` and
is fronted by a bearer token read from `~/.keisei/cortex.token`. CORS is
locked to a single origin provided at startup.

## Related

- parent: `kei-cortex/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
