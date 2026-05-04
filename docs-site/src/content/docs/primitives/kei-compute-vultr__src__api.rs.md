---
title: api.rs
path: kei-compute-vultr/src/api.rs
dna_hash: sha256:dd0c3a49970f0ae3
language: rust
size_loc: 242
generated: by-keidocs
---

# kei-compute-vultr/src/api.rs

Thin async client for the Vultr Cloud v2 API.

All requests carry `Authorization: Bearer $VULTR_API_KEY`. The wire
types here track the Vultr v2 schema (instances are wrapped in an
`instance` envelope on single-resource responses).

## Public API

- HTTP client for the Vultr v2 API.
- `pub fn new` — Build a client. `token` should be the value of `VULTR_API_KEY`.
- `pub fn with_base_url` — Override the API base — used by tests with `wiremock`.
- One of `os_id` or `iso_id` is required by Vultr; we expose both.
- Vultr requires user_data be base64-encoded.

## Related

- parent: `kei-compute-vultr/Cargo.toml`
- imports: crate, serde, wiremock

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
