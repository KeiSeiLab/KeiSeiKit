---
title: client.rs
path: kei-backend-daytona/src/client.rs
dna_hash: sha256:abe58fde9bf0d278
language: rust
size_loc: 200
generated: by-keidocs
---

# kei-backend-daytona/src/client.rs

Thin async REST client for Daytona.

No Rust SDK exists upstream → we hit the public REST surface directly.
All CRUD calls are timeout-bounded; 429/503 are retried with exponential
backoff (max 3 retries, base 250ms).

Architecture note — two API surfaces:
1. Management API  (`base_url`)   — sandbox CRUD (`/sandbox/...`).
2. Toolbox API     (per-sandbox)  — exec and file I/O.
Base URL fetched via `GET /sandbox/{id}/toolbox-proxy-url`,
cached per sandbox in `toolbox_cache`. See `toolbox.rs`.

## Public API

- Async Daytona REST client.
- `pub fn new` — Build a new client with the default 30s per-request timeout.
- `pub fn with_timeout` — Build a client with a custom per-request timeout.
- GET /sandbox/{name} — returns `None` on 404, otherwise the sandbox.
- GET /sandbox — enumerate all sandboxes for cost-guard quota counting.
- POST /sandbox — create a new sandbox from `spec`.
- POST /sandbox/{name}/start — resume a stopped/hibernated sandbox.
- POST /sandbox/{name}/stop — preserve filesystem.
- DELETE /sandbox/{name} — destroy filesystem too.
- Execute a command inside the sandbox via the Toolbox API.
- Upload a file to the sandbox via the Toolbox API.
- Download a file from the sandbox via the Toolbox API.
- Resolve the toolbox proxy URL for `sandbox_id` (cached after first fetch).
- Build a request with bearer auth + JSON accept.
- Send with retry on 429/503.

## Related

- parent: `kei-backend-daytona/Cargo.toml`
- imports: crate, reqwest, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
