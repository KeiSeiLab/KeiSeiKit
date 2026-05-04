---
title: api.rs
path: kei-compute-linode/src/api.rs
dna_hash: sha256:57cc6f0c7006b340
language: rust
size_loc: 234
generated: by-keidocs
---

# kei-compute-linode/src/api.rs

Linode v4 REST API client. Thin wrapper over `reqwest::Client` —
one method per provider verb. Wire types live alongside.

## Public API

- Linode HTTP client. Holds bearer token + base URL (overridable for tests).
- `pub fn new` — Construct from explicit token. For prod, prefer
- `pub fn from_env` — Read `LINODE_TOKEN` from environment.
- `pub fn with_base_url` — Override the base URL (test injection).
- `POST /linode/instances` — create instance.
- `GET /linode/instances/{id}` — read instance.
- `DELETE /linode/instances/{id}` — destroy.
- `POST /linode/instances/{id}/boot`
- `POST /linode/instances/{id}/shutdown`
- `POST /linode/instances/{id}/resize` — change tier slug.
- `POST /linode/instances` body. `metadata.user_data` carries the
- Base64-encoded cloud-init user-data.
- `GET /linode/instances/{id}` response (subset we use).

## Related

- parent: `kei-compute-linode/Cargo.toml`
- imports: crate, serde, wiremock

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
