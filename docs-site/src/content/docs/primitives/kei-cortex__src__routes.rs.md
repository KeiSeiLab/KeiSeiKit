---
title: routes.rs
path: kei-cortex/src/routes.rs
dna_hash: sha256:04841a726bd69175
language: rust
size_loc: 126
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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
