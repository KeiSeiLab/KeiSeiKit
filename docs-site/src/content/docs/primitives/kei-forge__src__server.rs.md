---
title: server.rs
path: kei-forge/src/server.rs
dna_hash: sha256:53cc7fc7bc14f4e4
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-forge/src/server.rs

Axum router — GET / (HTML form) and POST /forge (scaffold handler).

Intentionally stateless: no `AppState`, no handles, no async init.
Every request is self-contained. This lets tests spin up `app()` in
an ephemeral Tokio runtime without setup teardown overhead.

Security layers applied as middleware (see `crate::middleware`):
1. `require_local_host` — reject non-127.0.0.1 Host headers (blocks
DNS rebinding).
2. `require_json_content_type` — reject urlencoded POSTs (blocks
`<form>`-based CSRF).

GET / responses additionally carry CSP + nosniff + frame-deny headers.

## Public API

- `pub fn app` — Build the router. Called by `main.rs` and by tests.

## Related

- parent: `kei-forge/Cargo.toml`
- imports: axum, crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
