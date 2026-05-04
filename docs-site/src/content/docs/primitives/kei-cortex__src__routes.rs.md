---
title: routes.rs
path: kei-cortex/src/routes.rs
dna_hash: sha256:ad76fb9b7f827f3c
language: rust
size_loc: 127
generated: by-keidocs
---

# kei-cortex/src/routes.rs

Router assembly + CORS layer.

`/healthz` is mounted OUTSIDE the auth middleware so monitors can hit it
without a token. Everything under `/api` goes through `require_bearer`
(defined in `routes_auth`).

Per-route concurrency caps protect us from a runaway client draining our
upstream budget — `fal.ai` in particular bills per run, so we cap
`/portrait/stylize` at 2 concurrent installs system-wide. Other expensive
routes (`/tts`, `/stt`, `/chat`) get matching caps tuned to their bottleneck.

## Public API

- `pub fn build_router` — Build the top-level router. `cors_origin` must have been validated at
- Assemble the protected API sub-router (no auth layer yet — applied by caller).
- Build the CORS layer locked to a single origin.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, tower, tower_http

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
