---
title: client.rs
path: kei-compute-digitalocean/src/client.rs
dna_hash: sha256:d371ca58a9819d45
language: rust
size_loc: 184
generated: by-keidocs
---

# kei-compute-digitalocean/src/client.rs

Thin async REST v2 client for DigitalOcean.

No upstream Rust SDK is used — we hit the public surface directly
(`https://api.digitalocean.com/v2`) with bearer-token auth read from
`DIGITALOCEAN_TOKEN`. Base URL is overridable for `wiremock` tests.

## Public API

- `pub const DEFAULT_BASE_URL` — Default REST root.
- `pub const DEFAULT_TIMEOUT_SECS` — Per-request timeout.
- Spec passed to [`DigitalOceanClient::create_droplet`].
- Subset of the DigitalOcean droplet object we depend on.
- REST client. Cheap to clone (`Arc` inside `reqwest::Client`).
- `pub fn new` — Build with explicit token + base URL (use [`DEFAULT_BASE_URL`] in prod).
- `pub fn from_env` — Read `DIGITALOCEAN_TOKEN` from env, default base URL.
- POST /droplets — returns the freshly-created droplet (status `new`).
- POST /droplets/{id}/actions — `power_on`. 201 expected.
- POST /droplets/{id}/actions — `shutdown`. 201 expected.
- DELETE /droplets/{id} — 204 expected.
- GET /droplets/{id} — `Error::NotFound` on 404.
- GET /droplets — list all droplets the token can see.

## Related

- parent: `kei-compute-digitalocean/Cargo.toml`
- imports: crate, reqwest, serde, std

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
